import SwiftUI

struct MenuBarView: View {
    @ObservedObject var manager: FocusSessionManager

    /// Target for the progress ring (50 min Pomodoro-style)
    private let targetSeconds: CGFloat = 50 * 60

    var body: some View {
        VStack(spacing: 0) {
            // Header
            header
                .padding(.bottom, 16)

            // Focus Ring
            focusRing
                .padding(.bottom, 20)

            // Stats Row
            statsRow
                .padding(.bottom, 20)

            // Action Buttons
            actionButtons

            // State Badge
            stateBadge
                .padding(.top, 12)
        }
        .padding(20)
        .frame(width: 300)
        .animation(.spring(response: 0.5, dampingFraction: 0.8), value: manager.state)
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
            VStack(spacing: 4) {
                Text("CURRENT")
                    .font(.system(size: 9, weight: .semibold, design: .rounded))
                    .tracking(1.5)
                    .foregroundStyle(.secondary)

                Text(manager.currentFocusSeconds.asAdaptiveTime())
                    .font(.system(size: 28, weight: .bold, design: .rounded))
                    .foregroundStyle(
                        LinearGradient(
                            colors: timerGradient,
                            startPoint: .topLeading,
                            endPoint: .bottomTrailing
                        )
                    )
                    .contentTransition(.numericText())
                    .animation(.default, value: manager.currentFocusSeconds)

                if manager.isActive {
                    Text("of \(Int(targetSeconds / 60))m goal")
                        .font(.system(size: 10, weight: .medium, design: .rounded))
                        .foregroundStyle(.secondary.opacity(0.7))
                }
            }
        }
    }

    // MARK: - Stats Row

    private var statsRow: some View {
        HStack(spacing: 0) {
            statCard(
                icon: "clock.fill",
                label: "Session",
                value: manager.totalSessionSeconds.asAdaptiveTime(),
                color: .blue
            )
            Divider()
                .frame(height: 32)
                .padding(.horizontal, 8)
            statCard(
                icon: "exclamationmark.triangle.fill",
                label: "Interruptions",
                value: "\(manager.interruptionCount)",
                color: manager.interruptionCount > 0 ? .orange : .green
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

    // MARK: - State Badge

    private var stateBadge: some View {
        HStack(spacing: 6) {
            Circle()
                .fill(badgeColor)
                .frame(width: 6, height: 6)
                .shadow(color: badgeColor.opacity(0.6), radius: 3)

            Text(badgeLabel)
                .font(.system(size: 10, weight: .medium, design: .rounded))
                .foregroundStyle(.secondary)
        }
        .padding(.horizontal, 10)
        .padding(.vertical, 4)
        .background(
            Capsule()
                .fill(Color.primary.opacity(0.04))
        )
    }

    // MARK: - Computed Helpers

    private var ringProgress: CGFloat {
        guard targetSeconds > 0 else { return 0 }
        return min(CGFloat(manager.currentFocusSeconds) / targetSeconds, 1.0)
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

    private var badgeColor: Color {
        switch manager.state {
        case .running:    return .green
        case .interrupted: return .orange
        case .idle:       return .gray
        }
    }

    private var badgeLabel: String {
        switch manager.state {
        case .running:    return "Focusing"
        case .interrupted: return "Interrupted"
        case .idle:       return "Idle"
        }
    }
}
