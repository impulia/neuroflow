import SwiftUI

struct MenuBarView: View {
    @ObservedObject var manager: FocusSessionManager
    @State private var goalText: String = ""
    @State private var isEditing: Bool = false
    @FocusState private var isFieldFocused: Bool
    @Environment(\.scenePhase) private var scenePhase

    var body: some View {
        VStack(spacing: 0) {
            header
                .padding(.bottom, 16)

            focusRing
                .padding(.bottom, 20)

            statsRow
                .padding(.bottom, 20)
                .onTapGesture { commitGoal() }

            actionButtons
        }
        .padding(20)
        .frame(width: 300)
        .animation(.spring(response: 0.5, dampingFraction: 0.8), value: manager.state)
        .onAppear { goalText = "\(manager.goalMinutes)" }
        .onChange(of: manager.goalMinutes) { _, newValue in
            if !isEditing { goalText = "\(newValue)" }
        }
        .onChange(of: scenePhase) { _, newPhase in
            if newPhase != .active { commitGoal() }
        }
    }

    // MARK: - Header

    private var header: some View {
        HStack {
            Image(systemName: "brain.head.profile")
                .font(.title3)
                .foregroundStyle(
                    LinearGradient(
                        colors: [.purple, .cyan],
                        startPoint: .topLeading,
                        endPoint: .bottomTrailing
                    )
                )
            Text("Neuroflow")
                .font(.headline)
                .foregroundStyle(.primary)
            Spacer()
            SettingsLink {
                Image(systemName: "gearshape")
                    .font(.body)
                    .foregroundStyle(.secondary)
            }
            .buttonStyle(.plain)
        }
    }

    // MARK: - Focus Ring

    private var focusRing: some View {
        ZStack {
            // Background track
            Circle()
                .stroke(Color.primary.opacity(0.08), lineWidth: 10)
                .frame(width: 140, height: 140)
                .contentShape(Circle())
                .onTapGesture { commitGoal() }

            // Progress arc
            Circle()
                .trim(from: 0, to: ringProgress)
                .stroke(
                    AngularGradient(
                        gradient: Gradient(colors: ringColors),
                        center: .center,
                        startAngle: .degrees(0),
                        endAngle: .degrees(360)
                    ),
                    style: StrokeStyle(lineWidth: 10, lineCap: .round)
                )
                .rotationEffect(.degrees(-90))
                .frame(width: 140, height: 140)
                .shadow(color: ringGlowColor.opacity(0.45), radius: 6)
                .animation(.easeInOut(duration: 0.6), value: ringProgress)

            // Center Content
            VStack(spacing: 2) {
                if manager.isActive {
                    Text(statusLabel)
                        .font(.system(size: 9, weight: .semibold, design: .rounded))
                        .tracking(1.5)
                        .foregroundStyle(statusColor)
                        .transition(.opacity)
                }

                if manager.isActive {
                    Text(manager.remainingSeconds.asAdaptiveTime())
                        .font(.system(size: 28, weight: .bold, design: .rounded))
                        .monospacedDigit()
                        .foregroundStyle(
                            LinearGradient(
                                colors: timerGradient,
                                startPoint: .topLeading,
                                endPoint: .bottomTrailing
                            )
                        )
                        .contentTransition(.numericText())
                        .animation(.default, value: manager.remainingSeconds)
                } else {
                    goalEditor
                }
            }
        }
    }

    // MARK: - Stats Row

    private var statsRow: some View {
        HStack(spacing: 0) {
            statCard(
                icon: "clock.fill",
                label: "Focus",
                value: manager.totalFocusSeconds.asAdaptiveTime(),
                color: .blue
            )
            Divider()
                .frame(height: 32)
                .padding(.horizontal, 8)
            statCard(
                icon: "pause.circle.fill",
                label: "Idle",
                value: manager.totalInterruptedSeconds.asAdaptiveTime(),
                color: .orange
            )
        }
        .padding(.vertical, 10)
        .padding(.horizontal, 12)
        .background(
            RoundedRectangle(cornerRadius: 12, style: .continuous)
                .fill(Color.primary.opacity(0.04))
        )
    }

    private func statCard(icon: String, label: String, value: String, color: Color) -> some View {
        HStack(spacing: 8) {
            Image(systemName: icon)
                .font(.caption)
                .foregroundStyle(color)
            VStack(alignment: .leading, spacing: 1) {
                Text(label)
                    .font(.system(size: 9, weight: .medium, design: .rounded))
                    .foregroundStyle(.secondary)
                Text(value)
                    .font(.system(size: 14, weight: .semibold, design: .rounded))
                    .monospacedDigit()
                    .foregroundStyle(.primary)
                    .contentTransition(.numericText())
            }
        }
        .frame(maxWidth: .infinity)
    }

    // MARK: - Action Buttons

    private var actionButtons: some View {
        HStack(spacing: 10) {
            // Primary: Start / Stop
            Button {
                commitGoal()
                if manager.isRunning {
                    manager.stop()
                } else {
                    manager.start()
                }
            } label: {
                HStack(spacing: 6) {
                    Image(systemName: manager.isRunning ? "stop.fill" : "play.fill")
                        .font(.caption)
                    Text(primaryButtonLabel)
                        .font(.system(size: 12, weight: .semibold, design: .rounded))
                }
                .frame(maxWidth: .infinity)
                .frame(height: 32)
            }
            .buttonStyle(.borderedProminent)
            .tint(primaryButtonColor)

            // Interrupt
            Button {
                commitGoal()
                manager.interrupt()
            } label: {
                HStack(spacing: 6) {
                    Image(systemName: "pause.fill")
                        .font(.caption)
                    Text("Interrupt")
                        .font(.system(size: 12, weight: .semibold, design: .rounded))
                }
                .frame(maxWidth: .infinity)
                .frame(height: 32)
            }
            .buttonStyle(.bordered)
            .disabled(!manager.isRunning)
        }
    }

    // MARK: - Goal Editor

    private var goalEditor: some View {
        VStack(spacing: 2) {
            if isEditing {
                TextField("", text: $goalText)
                    .font(.system(size: 28, weight: .bold, design: .rounded))
                    .foregroundStyle(.primary)
                    .multilineTextAlignment(.center)
                    .textFieldStyle(.plain)
                    .frame(width: 72)
                    .focused($isFieldFocused)
                    .onSubmit { commitGoal() }
                    .onExitCommand { commitGoal() }
                    .onChange(of: isFieldFocused) { _, focused in
                        if !focused { commitGoal() }
                    }
                    .onAppear { isFieldFocused = true }
            } else {
                Text(goalText)
                    .font(.system(size: 28, weight: .bold, design: .rounded))
                    .foregroundStyle(
                        LinearGradient(
                            colors: [.purple.opacity(0.5), .cyan.opacity(0.5)],
                            startPoint: .topLeading,
                            endPoint: .bottomTrailing
                        )
                    )
            }

            Text("min")
                .font(.system(size: 10, weight: .medium, design: .rounded))
                .foregroundStyle(.secondary.opacity(0.5))
        }
        .contentShape(Rectangle())
        .onTapGesture {
            guard !isEditing else { return }
            goalText = "\(manager.goalMinutes)"
            isEditing = true
        }
    }

    private func commitGoal() {
        guard isEditing else { return }
        if let value = Int(goalText), value >= 1, value <= 480 {
            manager.goalMinutes = value
        }
        goalText = "\(manager.goalMinutes)"
        isEditing = false
        isFieldFocused = false
    }

    // MARK: - Computed Helpers

    private var ringProgress: CGFloat {
        guard manager.goalSeconds > 0 else { return 0 }
        return min(CGFloat(manager.totalFocusSeconds) / CGFloat(manager.goalSeconds), 1.0)
    }

    private var ringColors: [Color] {
        switch manager.state {
        case .running:    return [.purple, .blue, .cyan, .mint]
        case .interrupted: return [.orange, .yellow, .orange]
        case .idle:       return [.gray.opacity(0.3), .gray.opacity(0.15)]
        }
    }

    private var ringGlowColor: Color {
        switch manager.state {
        case .running:    return .cyan
        case .interrupted: return .orange
        case .idle:       return .clear
        }
    }

    private var timerGradient: [Color] {
        switch manager.state {
        case .running:    return [.cyan, .mint]
        case .interrupted: return [.orange, .yellow]
        case .idle:       return [.gray, .gray]
        }
    }

    private var primaryButtonLabel: String {
        switch manager.state {
        case .idle:        return "Start Focus"
        case .running:     return "End Session"
        case .interrupted: return "Resume"
        }
    }

    private var primaryButtonColor: Color {
        switch manager.state {
        case .idle:        return .green
        case .running:     return .red
        case .interrupted: return .green
        }
    }

    private var statusLabel: String {
        switch manager.state {
        case .idle:        return ""
        case .running:     return "FOCUS"
        case .interrupted: return "IDLE"
        }
    }

    private var statusColor: Color {
        switch manager.state {
        case .idle:        return .clear
        case .running:     return .cyan
        case .interrupted: return .orange
        }
    }
}
