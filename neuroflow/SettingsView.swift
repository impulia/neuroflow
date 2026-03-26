import SwiftUI
import Carbon.HIToolbox

struct SettingsView: View {
    @ObservedObject var manager: FocusSessionManager

    @State private var autoDetect: Bool
    @State private var idleMinutes: Double
    @State private var ssHotkey: Hotkey
    @State private var intHotkey: Hotkey

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
                    Picker(selection: $ssHotkey) {
                        ForEach(Self.presetHotkeys, id: \.self) { hotkey in
                            Text(hotkey.displayString()).tag(hotkey)
                        }
                    } label: {
                        Label("Start / Stop", systemImage: "keyboard")
                    }

                    Picker(selection: $intHotkey) {
                        ForEach(Self.presetHotkeys, id: \.self) { hotkey in
                            Text(hotkey.displayString()).tag(hotkey)
                        }
                    } label: {
                        Label("Interrupt", systemImage: "keyboard")
                    }
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
        .onAppear { bringWindowToFront() }
    }

    // MARK: - Preset Hotkeys

    private static let presetHotkeys: [Hotkey] = [
        .empty,
        // ⌃⌥F — Control+Option+F
        Hotkey(keyCode: UInt16(kVK_ANSI_F), carbonModifiers: UInt32(controlKey) | UInt32(optionKey)),
        // ⌃⌥S — Control+Option+S
        Hotkey(keyCode: UInt16(kVK_ANSI_S), carbonModifiers: UInt32(controlKey) | UInt32(optionKey)),
        // ⌃⌥P — Control+Option+P
        Hotkey(keyCode: UInt16(kVK_ANSI_P), carbonModifiers: UInt32(controlKey) | UInt32(optionKey)),
        // ⌃⌥R — Control+Option+R
        Hotkey(keyCode: UInt16(kVK_ANSI_R), carbonModifiers: UInt32(controlKey) | UInt32(optionKey)),
        // ⌃⌥I — Control+Option+I
        Hotkey(keyCode: UInt16(kVK_ANSI_I), carbonModifiers: UInt32(controlKey) | UInt32(optionKey)),
        // ⌃⌥N — Control+Option+N
        Hotkey(keyCode: UInt16(kVK_ANSI_N), carbonModifiers: UInt32(controlKey) | UInt32(optionKey)),
        // ⌃⇧F — Control+Shift+F
        Hotkey(keyCode: UInt16(kVK_ANSI_F), carbonModifiers: UInt32(controlKey) | UInt32(shiftKey)),
        // ⌘⇧F — Command+Shift+F
        Hotkey(keyCode: UInt16(kVK_ANSI_F), carbonModifiers: UInt32(cmdKey) | UInt32(shiftKey)),
        // ⌃⌥Space — Control+Option+Space
        Hotkey(keyCode: UInt16(kVK_Space), carbonModifiers: UInt32(controlKey) | UInt32(optionKey)),
    ]

    // MARK: - Helpers

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
