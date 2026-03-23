import Foundation
import AppKit
import Combine
import IOKit.pwr_mgt

@MainActor
final class FocusSessionManager: ObservableObject {

    // MARK: - Published State

    @Published private(set) var state: SessionState = .idle
    @Published private(set) var currentFocusSeconds: Int = 0
    @Published private(set) var totalSessionSeconds: Int = 0
    @Published private(set) var interruptionCount: Int = 0

    // MARK: - Settings (persisted via UserDefaults)

    @Published var autoDetectIdle: Bool {
        didSet { UserDefaults.standard.set(autoDetectIdle, forKey: "autoDetectIdle") }
    }
    @Published var idleThresholdMinutes: Int {
        didSet { UserDefaults.standard.set(idleThresholdMinutes, forKey: "idleThresholdMinutes") }
    }
    @Published var startStopHotkey: Hotkey {
        didSet { persistHotkey(startStopHotkey, key: "startStopHotkey"); registerHotkeys() }
    }
    @Published var interruptHotkey: Hotkey {
        didSet { persistHotkey(interruptHotkey, key: "interruptHotkey"); registerHotkeys() }
    }

    var idleThresholdSeconds: Int { idleThresholdMinutes * 60 }

    var isRunning: Bool { state == .running }
    var isInterrupted: Bool { state == .interrupted }
    var isActive: Bool { state != .idle }

    // MARK: - Private

    private var tickTimer: Timer?
    private var idleCheckTimer: Timer?
    private var sessionStartDate: Date?
    private var segmentStartDate: Date?
    private var completedSegments: [FocusSegment] = []

    // MARK: - Init

    init() {
        let defaults = UserDefaults.standard
        self.autoDetectIdle = defaults.object(forKey: "autoDetectIdle") as? Bool ?? true
        self.idleThresholdMinutes = defaults.object(forKey: "idleThresholdMinutes") as? Int ?? 5
        self.startStopHotkey = Self.loadHotkey(key: "startStopHotkey") ?? .empty
        self.interruptHotkey = Self.loadHotkey(key: "interruptHotkey") ?? .empty

        startIdleDetection()
        registerHotkeys()
    }

    // MARK: - Actions

    func start() {
        switch state {
        case .idle:
            sessionStartDate = Date()
            segmentStartDate = Date()
            currentFocusSeconds = 0
            totalSessionSeconds = 0
            interruptionCount = 0
            completedSegments = []
            state = .running
            startTicking()

        case .interrupted:
            segmentStartDate = Date()
            currentFocusSeconds = 0
            state = .running
            startTicking()

        case .running:
            break
        }
    }

    func stop() {
        guard state != .idle else { return }
        finalizeCurrentSegment()

        if let start = sessionStartDate {
            let record = FocusSessionRecord(
                startDate: start,
                endDate: Date(),
                totalFocusSeconds: totalSessionSeconds,
                interruptionCount: interruptionCount,
                segments: completedSegments
            )
            SessionStore.shared.append(record)
        }

        resetSession()
    }

    func interrupt() {
        guard state == .running else { return }
        finalizeCurrentSegment()
        interruptionCount += 1
        currentFocusSeconds = 0
        state = .interrupted
        stopTicking()
    }

    func toggleStartStop() {
        if state == .idle || state == .interrupted {
            start()
        } else {
            stop()
        }
    }

    // MARK: - Tick Timer

    private func startTicking() {
        stopTicking()
        tickTimer = Timer.scheduledTimer(withTimeInterval: 1.0, repeats: true) { _ in
            Task { @MainActor [weak self] in
                guard let self, self.state == .running else { return }
                self.currentFocusSeconds += 1
                self.totalSessionSeconds += 1
            }
        }
        RunLoop.main.add(tickTimer!, forMode: .common)
    }

    private func stopTicking() {
        tickTimer?.invalidate()
        tickTimer = nil
    }

    // MARK: - Segment Management

    private func finalizeCurrentSegment() {
        stopTicking()
        guard let start = segmentStartDate, currentFocusSeconds > 0 else {
            segmentStartDate = nil
            return
        }
        let segment = FocusSegment(startDate: start, endDate: Date())
        completedSegments.append(segment)
        segmentStartDate = nil
    }

    private func resetSession() {
        stopTicking()
        state = .idle
        currentFocusSeconds = 0
        totalSessionSeconds = 0
        interruptionCount = 0
        sessionStartDate = nil
        segmentStartDate = nil
        completedSegments = []
    }

    // MARK: - Idle Detection

    private func startIdleDetection() {
        idleCheckTimer = Timer.scheduledTimer(withTimeInterval: 2.0, repeats: true) { _ in
            Task { @MainActor [weak self] in
                guard let self, self.autoDetectIdle else { return }
                let idle = self.systemIdleSeconds()

                switch self.state {
                case .running:
                    if idle >= self.idleThresholdSeconds {
                        self.interrupt()
                    }
                case .interrupted:
                    if idle < 3 {
                        self.start()
                    }
                case .idle:
                    break
                }
            }
        }
        RunLoop.main.add(idleCheckTimer!, forMode: .common)
    }

    private func systemIdleSeconds() -> Int {
        var iterator: io_iterator_t = 0
        guard IOServiceGetMatchingServices(kIOMainPortDefault,
                                           IOServiceMatching("IOHIDSystem"),
                                           &iterator) == KERN_SUCCESS else { return 0 }
        let entry = IOIteratorNext(iterator)
        IOObjectRelease(iterator)
        guard entry != 0 else { return 0 }
        defer { IOObjectRelease(entry) }

        guard let prop = IORegistryEntryCreateCFProperty(
            entry,
            "HIDIdleTime" as CFString,
            kCFAllocatorDefault,
            0
        ) else { return 0 }

        let nanos = (prop.takeRetainedValue() as? NSNumber)?.uint64Value ?? 0
        return Int(nanos / 1_000_000_000)
    }

    // MARK: - Hotkey Persistence

    private func persistHotkey(_ hotkey: Hotkey, key: String) {
        if let data = try? JSONEncoder().encode(hotkey) {
            UserDefaults.standard.set(data, forKey: key)
        }
    }

    private static func loadHotkey(key: String) -> Hotkey? {
        guard let data = UserDefaults.standard.data(forKey: key) else { return nil }
        return try? JSONDecoder().decode(Hotkey.self, from: data)
    }

    private func registerHotkeys() {
        HotkeyCenter.shared.register(startStop: startStopHotkey, interrupt: interruptHotkey)
        HotkeyCenter.shared.onStartStop = { [weak self] in
            Task { @MainActor in self?.toggleStartStop() }
        }
        HotkeyCenter.shared.onInterrupt = { [weak self] in
            Task { @MainActor in self?.interrupt() }
        }
    }
}

// MARK: - Session Persistence

final class SessionStore {
    static let shared = SessionStore()

    private let fileURL: URL

    private init() {
        let appSupport = FileManager.default.urls(for: .applicationSupportDirectory, in: .userDomainMask).first!
        let dir = appSupport.appendingPathComponent("neuroflow", isDirectory: true)
        try? FileManager.default.createDirectory(at: dir, withIntermediateDirectories: true)
        self.fileURL = dir.appendingPathComponent("sessions.json")
    }

    func append(_ record: FocusSessionRecord) {
        var records = loadAll()
        records.append(record)
        save(records)
    }

    func loadAll() -> [FocusSessionRecord] {
        guard FileManager.default.fileExists(atPath: fileURL.path) else { return [] }
        guard let data = try? Data(contentsOf: fileURL) else { return [] }
        let decoder = JSONDecoder()
        decoder.dateDecodingStrategy = .iso8601
        return (try? decoder.decode([FocusSessionRecord].self, from: data)) ?? []
    }

    private func save(_ records: [FocusSessionRecord]) {
        let encoder = JSONEncoder()
        encoder.dateEncodingStrategy = .iso8601
        encoder.outputFormatting = [.prettyPrinted, .sortedKeys]
        guard let data = try? encoder.encode(records) else { return }
        try? data.write(to: fileURL, options: .atomic)
    }
}
