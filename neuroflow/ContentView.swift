import SwiftUI

/// ContentView is unused — Neuroflow is a menu-bar-only application.
/// All interaction happens through MenuBarView and SettingsView.
struct ContentView: View {
    var body: some View {
        Text("Neuroflow runs in the menu bar.")
            .font(.body)
            .foregroundStyle(.secondary)
            .padding()
    }
}

