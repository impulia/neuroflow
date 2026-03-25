import SwiftUI

@main
struct neuroflowApp: App {
    @StateObject private var manager = FocusSessionManager()

    var body: some Scene {
        MenuBarExtra {
            MenuBarView(manager: manager)
        } label: {
            menuBarLabel
        }
        .menuBarExtraStyle(.window)

        Settings {
            SettingsView(manager: manager)
        }
    }

    private var menuBarLabel: some View {
        HStack(spacing: 4) {
            Image(systemName: menuBarIcon)
            if manager.isActive {
                Text(manager.remainingSeconds.asAdaptiveTime())
                    .font(.system(size: 11, weight: .medium, design: .rounded))
                    .monospacedDigit()
            }
        }
    }

    private var menuBarIcon: String {
        switch manager.state {
        case .running:     return "brain.head.profile.fill"
        case .interrupted: return "pause.circle.fill"
        case .idle:        return "brain.head.profile"
        }
    }
}
