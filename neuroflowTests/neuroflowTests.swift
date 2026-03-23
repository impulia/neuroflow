import Testing
import Foundation
import AppKit
@testable import neuroflow

// MARK: - FocusSegment Tests

struct FocusSegmentTests {

    @Test func durationIsComputedFromDates() {
        let start = Date(timeIntervalSince1970: 1000)
        let end = Date(timeIntervalSince1970: 1300)
        let segment = FocusSegment(startDate: start, endDate: end)
        #expect(segment.durationSeconds == 300)
    }

    @Test func zeroDurationWhenStartEqualsEnd() {
        let date = Date(timeIntervalSince1970: 5000)
        let segment = FocusSegment(startDate: date, endDate: date)
        #expect(segment.durationSeconds == 0)
    }

    @Test func idIsStableWhenProvided() {
        let id = UUID()
        let segment = FocusSegment(id: id, startDate: Date(), endDate: Date())
        #expect(segment.id == id)
    }

    @Test func autoGeneratesIdWhenOmitted() {
        let a = FocusSegment(startDate: Date(), endDate: Date())
        let b = FocusSegment(startDate: Date(), endDate: Date())
        #expect(a.id != b.id)
    }

    @Test func equatableComparesAllFields() {
        let id = UUID()
        let start = Date(timeIntervalSince1970: 0)
        let end = Date(timeIntervalSince1970: 60)
        let a = FocusSegment(id: id, startDate: start, endDate: end)
        let b = FocusSegment(id: id, startDate: start, endDate: end)
        #expect(a == b)
    }

    @Test func notEqualWithDifferentId() {
        let start = Date(timeIntervalSince1970: 0)
        let end = Date(timeIntervalSince1970: 60)
        let a = FocusSegment(id: UUID(), startDate: start, endDate: end)
        let b = FocusSegment(id: UUID(), startDate: start, endDate: end)
        #expect(a != b)
    }

    @Test func codableRoundTrip() throws {
        let segment = FocusSegment(
            startDate: Date(timeIntervalSince1970: 1000),
            endDate: Date(timeIntervalSince1970: 1300)
        )
        let encoder = JSONEncoder()
        encoder.dateEncodingStrategy = .iso8601
        let data = try encoder.encode(segment)
        let decoder = JSONDecoder()
        decoder.dateDecodingStrategy = .iso8601
        let decoded = try decoder.decode(FocusSegment.self, from: data)
        #expect(decoded.id == segment.id)
        #expect(decoded.durationSeconds == segment.durationSeconds)
        #expect(decoded.startDate == segment.startDate)
        #expect(decoded.endDate == segment.endDate)
    }

    @Test func largeDurationSegment() {
        let start = Date(timeIntervalSince1970: 0)
        let end = Date(timeIntervalSince1970: 86400) // 24 hours
        let segment = FocusSegment(startDate: start, endDate: end)
        #expect(segment.durationSeconds == 86400)
    }
}

// MARK: - FocusSessionRecord Tests

struct FocusSessionRecordTests {

    private func makeRecord(
        startDate: Date = Date(timeIntervalSince1970: 1000),
        endDate: Date = Date(timeIntervalSince1970: 2000),
        totalFocusSeconds: Int = 900,
        interruptionCount: Int = 1,
        segments: [FocusSegment] = []
    ) -> FocusSessionRecord {
        FocusSessionRecord(
            startDate: startDate,
            endDate: endDate,
            totalFocusSeconds: totalFocusSeconds,
            interruptionCount: interruptionCount,
            segments: segments
        )
    }

    @Test func dayKeyFormatsCorrectly() {
        // 2026-03-23 in UTC
        let date = Date(timeIntervalSince1970: 1774483200)
        let formatter = DateFormatter()
        formatter.dateFormat = "yyyy-MM-dd"
        let expected = formatter.string(from: date)
        let record = makeRecord(startDate: date)
        #expect(record.dayKey == expected)
    }

    @Test func weekOfYearUsesCalendar() {
        let date = Date(timeIntervalSince1970: 1000)
        let expected = Calendar.current.component(.weekOfYear, from: date)
        let record = makeRecord(startDate: date)
        #expect(record.weekOfYear == expected)
    }

    @Test func yearUsesCalendar() {
        let date = Date(timeIntervalSince1970: 1000)
        let expected = Calendar.current.component(.year, from: date)
        let record = makeRecord(startDate: date)
        #expect(record.year == expected)
    }

    @Test func idIsStableWhenProvided() {
        let id = UUID()
        let record = FocusSessionRecord(
            id: id,
            startDate: Date(),
            endDate: Date(),
            totalFocusSeconds: 0,
            interruptionCount: 0,
            segments: []
        )
        #expect(record.id == id)
    }

    @Test func equatableWorks() {
        let id = UUID()
        let date = Date(timeIntervalSince1970: 0)
        let a = FocusSessionRecord(id: id, startDate: date, endDate: date, totalFocusSeconds: 100, interruptionCount: 0, segments: [])
        let b = FocusSessionRecord(id: id, startDate: date, endDate: date, totalFocusSeconds: 100, interruptionCount: 0, segments: [])
        #expect(a == b)
    }

    @Test func codableRoundTrip() throws {
        let segment = FocusSegment(
            startDate: Date(timeIntervalSince1970: 1000),
            endDate: Date(timeIntervalSince1970: 1300)
        )
        let record = FocusSessionRecord(
            startDate: Date(timeIntervalSince1970: 1000),
            endDate: Date(timeIntervalSince1970: 2000),
            totalFocusSeconds: 300,
            interruptionCount: 2,
            segments: [segment]
        )
        let encoder = JSONEncoder()
        encoder.dateEncodingStrategy = .iso8601
        let data = try encoder.encode(record)
        let decoder = JSONDecoder()
        decoder.dateDecodingStrategy = .iso8601
        let decoded = try decoder.decode(FocusSessionRecord.self, from: data)
        #expect(decoded.id == record.id)
        #expect(decoded.totalFocusSeconds == 300)
        #expect(decoded.interruptionCount == 2)
        #expect(decoded.segments.count == 1)
        #expect(decoded.segments[0].durationSeconds == 300)
    }

    @Test func zeroFocusSessionIsValid() {
        let record = makeRecord(totalFocusSeconds: 0, interruptionCount: 0)
        #expect(record.totalFocusSeconds == 0)
        #expect(record.interruptionCount == 0)
    }

    @Test func recordWithMultipleSegments() throws {
        let seg1 = FocusSegment(
            startDate: Date(timeIntervalSince1970: 100),
            endDate: Date(timeIntervalSince1970: 400)
        )
        let seg2 = FocusSegment(
            startDate: Date(timeIntervalSince1970: 500),
            endDate: Date(timeIntervalSince1970: 1000)
        )
        let record = FocusSessionRecord(
            startDate: Date(timeIntervalSince1970: 100),
            endDate: Date(timeIntervalSince1970: 1000),
            totalFocusSeconds: 800,
            interruptionCount: 1,
            segments: [seg1, seg2]
        )
        #expect(record.segments.count == 2)
        #expect(record.segments[0].durationSeconds == 300)
        #expect(record.segments[1].durationSeconds == 500)

        // Codable round-trip with multiple segments
        let encoder = JSONEncoder()
        encoder.dateEncodingStrategy = .iso8601
        let data = try encoder.encode(record)
        let decoder = JSONDecoder()
        decoder.dateDecodingStrategy = .iso8601
        let decoded = try decoder.decode(FocusSessionRecord.self, from: data)
        #expect(decoded.segments.count == 2)
    }
}

// MARK: - Hotkey Tests

struct HotkeyTests {

    @Test func emptyHotkeyProperties() {
        let empty = Hotkey.empty
        #expect(empty.keyCode == 0)
        #expect(empty.carbonModifiers == 0)
        #expect(empty.isEmpty)
    }

    @Test func nonEmptyHotkeyIsNotEmpty() {
        let hotkey = Hotkey(keyCode: 3, carbonModifiers: 256)
        #expect(!hotkey.isEmpty)
    }

    @Test func hotkeyWithOnlyKeyCodeIsNotEmpty() {
        let hotkey = Hotkey(keyCode: 5, carbonModifiers: 0)
        #expect(!hotkey.isEmpty)
    }

    @Test func hotkeyWithOnlyModifiersIsNotEmpty() {
        let hotkey = Hotkey(keyCode: 0, carbonModifiers: 256)
        #expect(!hotkey.isEmpty)
    }

    @Test func displayStringForEmpty() {
        #expect(Hotkey.empty.displayString() == "Not set")
    }

    @Test func displayStringWithCmdF() {
        // kVK_ANSI_F = 3, cmdKey = 256
        let hotkey = Hotkey(keyCode: 3, carbonModifiers: UInt32(256))
        let display = hotkey.displayString()
        #expect(display.contains("⌘"))
        #expect(display.contains("F"))
    }

    @Test func displayStringWithAllModifiers() {
        // controlKey = 4096, optionKey = 2048, shiftKey = 512, cmdKey = 256
        let mods = UInt32(4096 + 2048 + 512 + 256)
        let hotkey = Hotkey(keyCode: 0, carbonModifiers: mods) // kVK_ANSI_A = 0
        let display = hotkey.displayString()
        #expect(display.contains("⌃"))
        #expect(display.contains("⌥"))
        #expect(display.contains("⇧"))
        #expect(display.contains("⌘"))
        #expect(display.contains("A"))
    }

    @Test func displayStringForSpecialKeys() {
        // kVK_Space = 49
        let hotkey = Hotkey(keyCode: 49, carbonModifiers: UInt32(256))
        #expect(hotkey.displayString().contains("Space"))

        // kVK_Return = 36
        let returnKey = Hotkey(keyCode: 36, carbonModifiers: UInt32(256))
        #expect(returnKey.displayString().contains("↩"))

        // kVK_Tab = 48
        let tabKey = Hotkey(keyCode: 48, carbonModifiers: UInt32(256))
        #expect(tabKey.displayString().contains("⇥"))

        // kVK_Delete = 51
        let deleteKey = Hotkey(keyCode: 51, carbonModifiers: UInt32(256))
        #expect(deleteKey.displayString().contains("⌫"))
    }

    @Test func displayStringForFunctionKeys() {
        // kVK_F1 = 122
        let f1 = Hotkey(keyCode: 122, carbonModifiers: UInt32(256))
        #expect(f1.displayString().contains("F1"))

        // kVK_F12 = 111
        let f12 = Hotkey(keyCode: 111, carbonModifiers: UInt32(256))
        #expect(f12.displayString().contains("F12"))
    }

    @Test func displayStringForArrowKeys() {
        // kVK_LeftArrow = 123
        let left = Hotkey(keyCode: 123, carbonModifiers: UInt32(256))
        #expect(left.displayString().contains("←"))

        // kVK_RightArrow = 124
        let right = Hotkey(keyCode: 124, carbonModifiers: UInt32(256))
        #expect(right.displayString().contains("→"))

        // kVK_UpArrow = 126
        let up = Hotkey(keyCode: 126, carbonModifiers: UInt32(256))
        #expect(up.displayString().contains("↑"))

        // kVK_DownArrow = 125
        let down = Hotkey(keyCode: 125, carbonModifiers: UInt32(256))
        #expect(down.displayString().contains("↓"))
    }

    @Test func displayStringForUnknownKeyCode() {
        let hotkey = Hotkey(keyCode: 200, carbonModifiers: UInt32(256))
        #expect(hotkey.displayString().contains("(200)"))
    }

    @Test func displayStringForDigits() {
        // kVK_ANSI_0 = 29, kVK_ANSI_1 = 18, kVK_ANSI_9 = 25
        let zero = Hotkey(keyCode: 29, carbonModifiers: UInt32(256))
        #expect(zero.displayString().contains("0"))

        let one = Hotkey(keyCode: 18, carbonModifiers: UInt32(256))
        #expect(one.displayString().contains("1"))

        let nine = Hotkey(keyCode: 25, carbonModifiers: UInt32(256))
        #expect(nine.displayString().contains("9"))
    }

    @Test func codableRoundTrip() throws {
        let hotkey = Hotkey(keyCode: 3, carbonModifiers: 256)
        let data = try JSONEncoder().encode(hotkey)
        let decoded = try JSONDecoder().decode(Hotkey.self, from: data)
        #expect(decoded == hotkey)
    }

    @Test func equatable() {
        let a = Hotkey(keyCode: 3, carbonModifiers: 256)
        let b = Hotkey(keyCode: 3, carbonModifiers: 256)
        let c = Hotkey(keyCode: 4, carbonModifiers: 256)
        #expect(a == b)
        #expect(a != c)
    }
}

// MARK: - SessionState Tests

struct SessionStateTests {

    @Test func allCasesExist() {
        let states: [SessionState] = [.idle, .running, .interrupted]
        #expect(states.count == 3)
    }

    @Test func equatable() {
        #expect(SessionState.idle == SessionState.idle)
        #expect(SessionState.running == SessionState.running)
        #expect(SessionState.interrupted == SessionState.interrupted)
        #expect(SessionState.idle != SessionState.running)
        #expect(SessionState.running != SessionState.interrupted)
        #expect(SessionState.idle != SessionState.interrupted)
    }
}

// MARK: - Time Formatting Tests

struct TimeFormattingTests {

    @Test func asHMSFormatsCorrectly() {
        #expect(0.asHMS() == "00:00:00")
        #expect(1.asHMS() == "00:00:01")
        #expect(59.asHMS() == "00:00:59")
        #expect(60.asHMS() == "00:01:00")
        #expect(61.asHMS() == "00:01:01")
        #expect(3599.asHMS() == "00:59:59")
        #expect(3600.asHMS() == "01:00:00")
        #expect(3661.asHMS() == "01:01:01")
        #expect(86399.asHMS() == "23:59:59")
    }

    @Test func asMSFormatsCorrectly() {
        #expect(0.asMS() == "00:00")
        #expect(1.asMS() == "00:01")
        #expect(59.asMS() == "00:59")
        #expect(60.asMS() == "01:00")
        #expect(61.asMS() == "01:01")
        #expect(3599.asMS() == "59:59")
    }

    @Test func asAdaptiveTimeSwitchesAtOneHour() {
        // Under 1 hour → MM:SS
        #expect(0.asAdaptiveTime() == "00:00")
        #expect(59.asAdaptiveTime() == "00:59")
        #expect(3599.asAdaptiveTime() == "59:59")

        // At 1 hour and above → HH:MM:SS
        #expect(3600.asAdaptiveTime() == "01:00:00")
        #expect(3661.asAdaptiveTime() == "01:01:01")
        #expect(7200.asAdaptiveTime() == "02:00:00")
    }

    @Test func asMSOverflowsGracefully() {
        // 3600 seconds is 60 minutes, asMS should show 60:00
        #expect(3600.asMS() == "60:00")
    }
}

// MARK: - SessionStore Tests

struct SessionStoreTests {

    private func makeTempStore() -> SessionStore {
        let tempDir = FileManager.default.temporaryDirectory
            .appendingPathComponent(UUID().uuidString, isDirectory: true)
        try? FileManager.default.createDirectory(at: tempDir, withIntermediateDirectories: true)
        let fileURL = tempDir.appendingPathComponent("test_sessions.json")
        return SessionStore(fileURL: fileURL)
    }

    private func makeRecord(
        totalFocusSeconds: Int = 300,
        interruptionCount: Int = 0
    ) -> FocusSessionRecord {
        FocusSessionRecord(
            startDate: Date(timeIntervalSince1970: 1000),
            endDate: Date(timeIntervalSince1970: 1300),
            totalFocusSeconds: totalFocusSeconds,
            interruptionCount: interruptionCount,
            segments: [
                FocusSegment(
                    startDate: Date(timeIntervalSince1970: 1000),
                    endDate: Date(timeIntervalSince1970: 1300)
                )
            ]
        )
    }

    @Test func loadAllReturnsEmptyWhenNoFile() {
        let store = makeTempStore()
        let records = store.loadAll()
        #expect(records.isEmpty)
    }

    @Test func appendAndLoadRoundTrip() {
        let store = makeTempStore()
        let record = makeRecord()
        store.append(record)
        let loaded = store.loadAll()
        #expect(loaded.count == 1)
        #expect(loaded[0].id == record.id)
        #expect(loaded[0].totalFocusSeconds == 300)
        #expect(loaded[0].interruptionCount == 0)
        #expect(loaded[0].segments.count == 1)
    }

    @Test func appendAccumulates() {
        let store = makeTempStore()
        store.append(makeRecord(totalFocusSeconds: 100))
        store.append(makeRecord(totalFocusSeconds: 200))
        store.append(makeRecord(totalFocusSeconds: 300))
        let loaded = store.loadAll()
        #expect(loaded.count == 3)
        #expect(loaded[0].totalFocusSeconds == 100)
        #expect(loaded[1].totalFocusSeconds == 200)
        #expect(loaded[2].totalFocusSeconds == 300)
    }

    @Test func saveAndLoadPreservesAllFields() {
        let store = makeTempStore()
        let segment = FocusSegment(
            startDate: Date(timeIntervalSince1970: 5000),
            endDate: Date(timeIntervalSince1970: 5600)
        )
        let record = FocusSessionRecord(
            startDate: Date(timeIntervalSince1970: 5000),
            endDate: Date(timeIntervalSince1970: 6000),
            totalFocusSeconds: 600,
            interruptionCount: 3,
            segments: [segment]
        )
        store.append(record)
        let loaded = store.loadAll()
        #expect(loaded.count == 1)
        let r = loaded[0]
        #expect(r.id == record.id)
        #expect(r.startDate == record.startDate)
        #expect(r.endDate == record.endDate)
        #expect(r.totalFocusSeconds == 600)
        #expect(r.interruptionCount == 3)
        #expect(r.segments.count == 1)
        #expect(r.segments[0].id == segment.id)
        #expect(r.segments[0].startDate == segment.startDate)
        #expect(r.segments[0].endDate == segment.endDate)
        #expect(r.segments[0].durationSeconds == 600)
    }

    @Test func loadAllReturnsEmptyForCorruptFile() {
        let tempDir = FileManager.default.temporaryDirectory
            .appendingPathComponent(UUID().uuidString, isDirectory: true)
        try? FileManager.default.createDirectory(at: tempDir, withIntermediateDirectories: true)
        let fileURL = tempDir.appendingPathComponent("corrupt.json")
        try? "not valid json {{{".data(using: .utf8)?.write(to: fileURL)
        let store = SessionStore(fileURL: fileURL)
        let records = store.loadAll()
        #expect(records.isEmpty)
    }

    @Test func saveOverwritesThenReloads() {
        let store = makeTempStore()
        let records = [
            makeRecord(totalFocusSeconds: 100),
            makeRecord(totalFocusSeconds: 200),
        ]
        store.save(records)
        let loaded = store.loadAll()
        #expect(loaded.count == 2)
        #expect(loaded[0].totalFocusSeconds == 100)
        #expect(loaded[1].totalFocusSeconds == 200)
    }

    @Test func appendToExistingFile() {
        let store = makeTempStore()
        store.save([makeRecord(totalFocusSeconds: 100)])
        store.append(makeRecord(totalFocusSeconds: 200))
        let loaded = store.loadAll()
        #expect(loaded.count == 2)
    }

    @Test func zeroFocusSessionIsSaved() {
        let store = makeTempStore()
        let record = FocusSessionRecord(
            startDate: Date(timeIntervalSince1970: 1000),
            endDate: Date(timeIntervalSince1970: 1000),
            totalFocusSeconds: 0,
            interruptionCount: 0,
            segments: []
        )
        store.append(record)
        let loaded = store.loadAll()
        #expect(loaded.count == 1)
        #expect(loaded[0].totalFocusSeconds == 0)
    }

    @Test func datesAreStoredAsISO8601() throws {
        let tempDir = FileManager.default.temporaryDirectory
            .appendingPathComponent(UUID().uuidString, isDirectory: true)
        try FileManager.default.createDirectory(at: tempDir, withIntermediateDirectories: true)
        let fileURL = tempDir.appendingPathComponent("iso_test.json")
        let store = SessionStore(fileURL: fileURL)
        let record = makeRecord()
        store.append(record)

        let data = try Data(contentsOf: fileURL)
        let jsonString = String(data: data, encoding: .utf8)!
        // ISO 8601 dates contain "T" and end with "Z"
        #expect(jsonString.contains("T"))
        #expect(jsonString.contains("Z"))
    }
}

// MARK: - FocusSessionManager Tests

@MainActor
struct FocusSessionManagerTests {

    private func makeManager() -> (FocusSessionManager, SessionStore) {
        let tempDir = FileManager.default.temporaryDirectory
            .appendingPathComponent(UUID().uuidString, isDirectory: true)
        try? FileManager.default.createDirectory(at: tempDir, withIntermediateDirectories: true)
        let fileURL = tempDir.appendingPathComponent("test_sessions.json")
        let store = SessionStore(fileURL: fileURL)
        let manager = FocusSessionManager(sessionStore: store, enableTimers: false)
        return (manager, store)
    }

    // MARK: - Initial State

    @Test func initialStateIsIdle() {
        let (manager, _) = makeManager()
        #expect(manager.state == .idle)
        #expect(manager.currentFocusSeconds == 0)
        #expect(manager.totalSessionSeconds == 0)
        #expect(manager.interruptionCount == 0)
    }

    @Test func initialComputedProperties() {
        let (manager, _) = makeManager()
        #expect(!manager.isRunning)
        #expect(!manager.isInterrupted)
        #expect(!manager.isActive)
    }

    // MARK: - Start

    @Test func startFromIdleSetsRunning() {
        let (manager, _) = makeManager()
        manager.start()
        #expect(manager.state == .running)
        #expect(manager.isRunning)
        #expect(manager.isActive)
        #expect(!manager.isInterrupted)
    }

    @Test func startFromIdleResetsCounters() {
        let (manager, _) = makeManager()
        manager.start()
        #expect(manager.currentFocusSeconds == 0)
        #expect(manager.totalSessionSeconds == 0)
        #expect(manager.interruptionCount == 0)
    }

    @Test func startFromIdleSetsSessionStartDate() {
        let (manager, _) = makeManager()
        manager.start()
        #expect(manager.sessionStartDate != nil)
        #expect(manager.segmentStartDate != nil)
    }

    @Test func startWhileRunningIsNoOp() {
        let (manager, _) = makeManager()
        manager.start()
        let startDate = manager.sessionStartDate
        manager.start() // should be no-op
        #expect(manager.state == .running)
        #expect(manager.sessionStartDate == startDate)
    }

    // MARK: - Stop

    @Test func stopFromIdleIsNoOp() {
        let (manager, store) = makeManager()
        manager.stop()
        #expect(manager.state == .idle)
        #expect(store.loadAll().isEmpty)
    }

    @Test func stopFromRunningSavesAndResets() {
        let (manager, store) = makeManager()
        manager.start()
        manager.stop()
        #expect(manager.state == .idle)
        #expect(manager.currentFocusSeconds == 0)
        #expect(manager.totalSessionSeconds == 0)
        #expect(manager.interruptionCount == 0)
        #expect(manager.sessionStartDate == nil)
        #expect(manager.segmentStartDate == nil)
        #expect(manager.completedSegments.isEmpty)
        #expect(store.loadAll().count == 1)
    }

    @Test func stopFromInterruptedSavesAndResets() {
        let (manager, store) = makeManager()
        manager.start()
        manager.interrupt()
        manager.stop()
        #expect(manager.state == .idle)
        #expect(store.loadAll().count == 1)
        let record = store.loadAll()[0]
        #expect(record.interruptionCount == 1)
    }

    // MARK: - Interrupt

    @Test func interruptFromRunningSetsInterrupted() {
        let (manager, _) = makeManager()
        manager.start()
        manager.interrupt()
        #expect(manager.state == .interrupted)
        #expect(manager.isInterrupted)
        #expect(manager.isActive)
        #expect(!manager.isRunning)
    }

    @Test func interruptIncrementsCount() {
        let (manager, _) = makeManager()
        manager.start()
        manager.interrupt()
        #expect(manager.interruptionCount == 1)
    }

    @Test func interruptResetsCurrentFocusSeconds() {
        let (manager, _) = makeManager()
        manager.start()
        manager.interrupt()
        #expect(manager.currentFocusSeconds == 0)
    }

    @Test func interruptFromIdleIsNoOp() {
        let (manager, _) = makeManager()
        manager.interrupt()
        #expect(manager.state == .idle)
        #expect(manager.interruptionCount == 0)
    }

    @Test func interruptFromInterruptedIsNoOp() {
        let (manager, _) = makeManager()
        manager.start()
        manager.interrupt()
        manager.interrupt() // should be no-op
        #expect(manager.state == .interrupted)
        #expect(manager.interruptionCount == 1) // not incremented again
    }

    // MARK: - Resume (start from interrupted)

    @Test func resumeFromInterruptedSetsRunning() {
        let (manager, _) = makeManager()
        manager.start()
        manager.interrupt()
        manager.start() // resume
        #expect(manager.state == .running)
        #expect(manager.currentFocusSeconds == 0)
    }

    @Test func resumePreservesSessionStartDate() {
        let (manager, _) = makeManager()
        manager.start()
        let original = manager.sessionStartDate
        manager.interrupt()
        manager.start() // resume
        #expect(manager.sessionStartDate == original)
    }

    @Test func resumeSetsNewSegmentStartDate() {
        let (manager, _) = makeManager()
        manager.start()
        let originalSegment = manager.segmentStartDate
        manager.interrupt()
        manager.start() // resume
        #expect(manager.segmentStartDate != nil)
        #expect(manager.segmentStartDate != originalSegment)
    }

    // MARK: - Toggle

    @Test func toggleFromIdleStarts() {
        let (manager, _) = makeManager()
        manager.toggleStartStop()
        #expect(manager.state == .running)
    }

    @Test func toggleFromRunningStops() {
        let (manager, _) = makeManager()
        manager.start()
        manager.toggleStartStop()
        #expect(manager.state == .idle)
    }

    @Test func toggleFromInterruptedResumes() {
        let (manager, _) = makeManager()
        manager.start()
        manager.interrupt()
        manager.toggleStartStop()
        #expect(manager.state == .running)
    }

    // MARK: - Multi-interrupt Cycle

    @Test func multipleInterruptCycles() {
        let (manager, store) = makeManager()
        manager.start()
        manager.interrupt() // 1st
        manager.start()
        manager.interrupt() // 2nd
        manager.start()
        manager.interrupt() // 3rd
        #expect(manager.interruptionCount == 3)

        manager.start() // resume
        manager.stop()
        #expect(manager.state == .idle)
        let records = store.loadAll()
        #expect(records.count == 1)
        #expect(records[0].interruptionCount == 3)
    }

    // MARK: - Session Record Content

    @Test func savedRecordHasCorrectStructure() {
        let (manager, store) = makeManager()
        manager.start()
        manager.stop()
        let records = store.loadAll()
        #expect(records.count == 1)
        let record = records[0]
        #expect(record.startDate <= record.endDate)
        #expect(record.totalFocusSeconds == 0)
        #expect(record.interruptionCount == 0)
    }

    // MARK: - Settings Defaults

    @Test func defaultIdleThreshold() {
        let (manager, _) = makeManager()
        // Default is 5 min unless UserDefaults has been set before
        #expect(manager.idleThresholdSeconds == manager.idleThresholdMinutes * 60)
    }

    @Test func idleThresholdSecondsComputed() {
        let (manager, _) = makeManager()
        manager.idleThresholdMinutes = 10
        #expect(manager.idleThresholdSeconds == 600)
    }

    // MARK: - State Machine Completeness

    @Test func fullLifecycleFromIdleAndBack() {
        let (manager, store) = makeManager()
        // idle → running → interrupted → running → stop → idle
        #expect(manager.state == .idle)
        manager.start()
        #expect(manager.state == .running)
        manager.interrupt()
        #expect(manager.state == .interrupted)
        manager.start()
        #expect(manager.state == .running)
        manager.stop()
        #expect(manager.state == .idle)
        #expect(store.loadAll().count == 1)
    }

    @Test func stopTwiceIsIdempotent() {
        let (manager, store) = makeManager()
        manager.start()
        manager.stop()
        manager.stop() // second stop from idle
        #expect(manager.state == .idle)
        #expect(store.loadAll().count == 1)
    }

    @Test func startStopStartCreatesNewSession() {
        let (manager, store) = makeManager()
        manager.start()
        manager.stop()
        #expect(store.loadAll().count == 1)
        manager.start()
        manager.stop()
        #expect(store.loadAll().count == 2)
    }
}

// MARK: - HotkeyCenter Helper Tests

struct HotkeyCenterHelperTests {

    @Test func carbonModifiersFromNSEventFlags() {
        let cmdFlags: NSEvent.ModifierFlags = .command
        let result = HotkeyCenter.carbonModifiers(from: cmdFlags)
        #expect(result == UInt32(256)) // cmdKey

        let shiftFlags: NSEvent.ModifierFlags = .shift
        let shiftResult = HotkeyCenter.carbonModifiers(from: shiftFlags)
        #expect(shiftResult == UInt32(512)) // shiftKey

        let optionFlags: NSEvent.ModifierFlags = .option
        let optionResult = HotkeyCenter.carbonModifiers(from: optionFlags)
        #expect(optionResult == UInt32(2048)) // optionKey

        let controlFlags: NSEvent.ModifierFlags = .control
        let controlResult = HotkeyCenter.carbonModifiers(from: controlFlags)
        #expect(controlResult == UInt32(4096)) // controlKey
    }

    @Test func carbonModifiersCombined() {
        let flags: NSEvent.ModifierFlags = [.command, .shift, .option, .control]
        let result = HotkeyCenter.carbonModifiers(from: flags)
        let expected = UInt32(256 + 512 + 2048 + 4096)
        #expect(result == expected)
    }

    @Test func carbonModifiersEmptyFlags() {
        let result = HotkeyCenter.carbonModifiers(from: [])
        #expect(result == 0)
    }
}
