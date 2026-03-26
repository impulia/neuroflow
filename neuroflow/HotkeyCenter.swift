import AppKit
import Carbon.HIToolbox

/// Manages global hotkey registration using the Carbon Event API.
final class HotkeyCenter {
    static let shared = HotkeyCenter()

    var onStartStop: (() -> Void)?
    var onInterrupt: (() -> Void)?

    private var startStopRef: EventHotKeyRef?
    private var interruptRef: EventHotKeyRef?
    private var eventHandler: EventHandlerRef?

    private let startStopID = EventHotKeyID(
        signature: fourCharCode("nfss"),
        id: 1
    )
    private let interruptID = EventHotKeyID(
        signature: fourCharCode("nfit"),
        id: 2
    )

    private init() {
        installCarbonHandler()
    }

    deinit {
        unregisterAll()
        if let handler = eventHandler {
            RemoveEventHandler(handler)
        }
    }

    func register(startStop: Hotkey, interrupt: Hotkey) {
        unregisterAll()

        if !startStop.isEmpty {
            var ref: EventHotKeyRef?
            RegisterEventHotKey(
                UInt32(startStop.keyCode),
                startStop.carbonModifiers,
                startStopID,
                GetEventDispatcherTarget(),
                0,
                &ref
            )
            startStopRef = ref
        }

        if !interrupt.isEmpty {
            var ref: EventHotKeyRef?
            RegisterEventHotKey(
                UInt32(interrupt.keyCode),
                interrupt.carbonModifiers,
                interruptID,
                GetEventDispatcherTarget(),
                0,
                &ref
            )
            interruptRef = ref
        }
    }

    func unregisterAll() {
        if let ref = startStopRef { UnregisterEventHotKey(ref); startStopRef = nil }
        if let ref = interruptRef { UnregisterEventHotKey(ref); interruptRef = nil }
    }

    // MARK: - Carbon Event Handler

    private func installCarbonHandler() {
        var eventType = EventTypeSpec(
            eventClass: OSType(kEventClassKeyboard),
            eventKind: UInt32(kEventHotKeyPressed)
        )

        let selfPtr = UnsafeMutableRawPointer(Unmanaged.passUnretained(self).toOpaque())

        let callback: EventHandlerUPP = { _, event, userData -> OSStatus in
            guard let event else { return OSStatus(eventNotHandledErr) }
            var hotKeyID = EventHotKeyID()
            let status = GetEventParameter(
                event,
                EventParamName(kEventParamDirectObject),
                EventParamType(typeEventHotKeyID),
                nil,
                MemoryLayout<EventHotKeyID>.size,
                nil,
                &hotKeyID
            )
            guard status == noErr else { return status }

            let center = Unmanaged<HotkeyCenter>.fromOpaque(userData!).takeUnretainedValue()
            if hotKeyID.id == center.startStopID.id {
                center.onStartStop?()
            } else if hotKeyID.id == center.interruptID.id {
                center.onInterrupt?()
            }
            return noErr
        }

        InstallEventHandler(
            GetEventDispatcherTarget(),
            callback,
            1,
            &eventType,
            selfPtr,
            &eventHandler
        )
    }

    // MARK: - Modifier Helpers

    static func carbonModifiers(from flags: NSEvent.ModifierFlags) -> UInt32 {
        var m: UInt32 = 0
        if flags.contains(.control) { m |= UInt32(controlKey) }
        if flags.contains(.option)  { m |= UInt32(optionKey) }
        if flags.contains(.shift)   { m |= UInt32(shiftKey) }
        if flags.contains(.command) { m |= UInt32(cmdKey) }
        return m
    }
}

private func fourCharCode(_ s: String) -> OSType {
    s.utf8.prefix(4).reduce(0) { ($0 << 8) | OSType($1) }
}
