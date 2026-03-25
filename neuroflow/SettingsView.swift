import SwiftUI
import Carbon.HIToolbox

struct SettingsView: View {
    @ObservedObject var manager: FocusSessionManager

    @State private var autoDetect: Bool
    @State private var idleMinutes: Double
    @State private var ssHotkey: Hotkey
    @State private var intHotkey: Hotkey
    @State private var recording: RecordTarget? = nil

    private enum RecordTarget { case startStop, interrupt }

    init(manager: FocusSessionManager) {
        self.manager = manager
        _autoDetect = State(initialValue: manager.autoDetectIdle)
        _idleMinutes = State(initialValue: Double(manager.idleThresholdMinutes))
        _ssHotkey = State(initialValue: manager.startStopHotkey)
        _intHotkey = State(initialValue: manager.interruptHotkey)
    }

    var body: some View {
        VStack(spacing: 0) {
            Form {
                // MARK: - Idle Detection
                Section {
                    Toggle(isOn: $autoDetect) {
                        Label("Auto-detect interruptions", systemImage: "timer")
                    }

                    if autoDetect {
                        LabeledContent {
                            Text("\(Int(idleMinutes)) min")
                                .foregroundStyle(.secondary)
                                .monospacedDigit()
                        } label: {
                            Label("Idle threshold", systemImage: "hourglass.bottomhalf.filled")
                        }
                        Slider(value: $idleMinutes, in: 1...30, step: 1)
                            .tint(.cyan)
                    }
                } header: {
                    Text("Idle Detection")
                }

                // MARK: - Hotkeys
                Section {
                    hotkeyRow(label: "Start / Stop", hotkey: ssHotkey, target: .startStop)
                    hotkeyRow(label: "Interrupt", hotkey: intHotkey, target: .interrupt)
                } header: {
                    Text("Global Hotkeys")
                } footer: {
                    Text("Hotkeys work system-wide, even when Neuroflow is in the background.")
                }
            }
            .formStyle(.grouped)
            .scrollDisabled(true)

            HStack {
                Spacer()
                Button("Save") { save() }
                    .keyboardShortcut(.defaultAction)
                    .buttonStyle(.borderedProminent)
                    .tint(.cyan)
            }
            .padding(.horizontal, 20)
            .padding(.bottom, 16)
        }
        .frame(width: 480)
        .fixedSize(horizontal: false, vertical: true)
        .overlay { recordingOverlay }
        .onAppear { bringWindowToFront() }
    }

    // MARK: - Hotkey Row

    private func hotkeyRow(label: String, hotkey: Hotkey, target: RecordTarget) -> some View {
        LabeledContent {
            HStack(spacing: 8) {
                Text(hotkey.displayString())
                    .font(.system(.body, design: .rounded).weight(.medium))
                    .foregroundStyle(hotkey.isEmpty ? .secondary : .primary)
                    .padding(.horizontal, 10)
                    .padding(.vertical, 4)
                    .background(
                        RoundedRectangle(cornerRadius: 6, style: .continuous)
                            .fill(Color.primary.opacity(0.06))
                    )

                Button(recording == target ? "Cancel" : "Record") {
                    recording = recording == target ? nil : target
                }
                .font(.caption.weight(.medium))
                .buttonStyle(.bordered)
                .tint(recording == target ? .red : .accentColor)

                if !hotkey.isEmpty {
                    Button {
                        clearHotkey(target)
                    } label: {
                        Image(systemName: "xmark.circle.fill")
                            .foregroundStyle(.secondary)
                    }
                    .buttonStyle(.plain)
                }
            }
        } label: {
            Label(label, systemImage: "keyboard")
        }
    }

    // MARK: - Recording Overlay

    @ViewBuilder
    private var recordingOverlay: some View {
        if recording != nil {
            Color.black.opacity(0.5)
                .ignoresSafeArea()
                .overlay(
                    VStack(spacing: 14) {
                        Image(systemName: "keyboard")
                            .font(.largeTitle)
                            .foregroundStyle(
                                LinearGradient(colors: [.cyan, .purple], startPoint: .leading, endPoint: .trailing)
                            )
                        Text("Press your shortcut")
                            .font(.title3.weight(.semibold))
                            .foregroundStyle(.white)
                        Text(recording == .startStop ? "Start / Stop" : "Interrupt")
                            .font(.subheadline)
                            .foregroundStyle(.white.opacity(0.7))
                        Text("Press Esc to cancel")
                            .font(.caption)
                            .foregroundStyle(.white.opacity(0.4))
                    }
                    .padding(32)
                    .background(
                        RoundedRectangle(cornerRadius: 16, style: .continuous)
                            .fill(.ultraThinMaterial)
                            .shadow(color: .black.opacity(0.3), radius: 20)
                    )
                )
                .onKeyPress { keyPress in
                    handleKeyPress(keyPress)
                    return .handled
                }
        }
    }

    // MARK: - Key Handling

    private func handleKeyPress(_ keyPress: KeyPress) {
        if keyPress.key == .escape {
            recording = nil
            return
        }

        let carbonMods = carbonModifiers(from: keyPress.modifiers)
        let keyCode = keyPress.key.character.asciiValue.map { UInt16($0) } ?? 0

        guard carbonMods != 0 else { return }

        let hotkey = Hotkey(keyCode: keyCode, carbonModifiers: carbonMods)

        switch recording {
        case .startStop: ssHotkey = hotkey
        case .interrupt: intHotkey = hotkey
        case nil: break
        }
        recording = nil
    }

    private func clearHotkey(_ target: RecordTarget) {
        switch target {
        case .startStop: ssHotkey = .empty
        case .interrupt: intHotkey = .empty
        }
    }

    private func carbonModifiers(from mods: SwiftUI.EventModifiers) -> UInt32 {
        var m: UInt32 = 0
        if mods.contains(.command) { m |= UInt32(cmdKey) }
        if mods.contains(.shift)   { m |= UInt32(shiftKey) }
        if mods.contains(.option)  { m |= UInt32(optionKey) }
        if mods.contains(.control) { m |= UInt32(controlKey) }
        return m
    }

    private func bringWindowToFront() {
        NSApp.activate(ignoringOtherApps: true)
        Task { @MainActor in
            try? await Task.sleep(for: .milliseconds(100))
            if let window = NSApp.keyWindow {
                window.level = .floating
                window.orderFrontRegardless()
            }
        }
    }

    private func save() {
        manager.autoDetectIdle = autoDetect
        manager.idleThresholdMinutes = Int(idleMinutes)
        manager.startStopHotkey = ssHotkey
        manager.interruptHotkey = intHotkey

        NSApp.keyWindow?.close()
    }
}
