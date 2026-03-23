import Foundation
import Carbon.HIToolbox

// MARK: - Session Data Models

/// A single uninterrupted focus segment within a session.
struct FocusSegment: Codable, Equatable, Identifiable {
    let id: UUID
    let startDate: Date
    let endDate: Date
    let durationSeconds: Int

    init(id: UUID = UUID(), startDate: Date, endDate: Date) {
        self.id = id
        self.startDate = startDate
        self.endDate = endDate
        self.durationSeconds = Int(endDate.timeIntervalSince(startDate))
    }
}

/// A complete focus session record, persisted to disk.
/// Structured for future weekly/daily insight aggregation.
struct FocusSessionRecord: Codable, Identifiable, Equatable {
    let id: UUID
    let startDate: Date
    let endDate: Date
    let totalFocusSeconds: Int
    let interruptionCount: Int
    let segments: [FocusSegment]

    /// Calendar day string for grouping (e.g. "2026-03-23")
    var dayKey: String {
        let formatter = DateFormatter()
        formatter.dateFormat = "yyyy-MM-dd"
        return formatter.string(from: startDate)
    }

    /// ISO week number for weekly aggregation
    var weekOfYear: Int {
        Calendar.current.component(.weekOfYear, from: startDate)
    }

    /// Year for weekly aggregation
    var year: Int {
        Calendar.current.component(.year, from: startDate)
    }

    init(id: UUID = UUID(), startDate: Date, endDate: Date, totalFocusSeconds: Int, interruptionCount: Int, segments: [FocusSegment]) {
        self.id = id
        self.startDate = startDate
        self.endDate = endDate
        self.totalFocusSeconds = totalFocusSeconds
        self.interruptionCount = interruptionCount
        self.segments = segments
    }
}

// MARK: - Hotkey Model

struct Hotkey: Codable, Equatable {
    var keyCode: UInt16
    var carbonModifiers: UInt32

    static let empty = Hotkey(keyCode: 0, carbonModifiers: 0)

    var isEmpty: Bool { keyCode == 0 && carbonModifiers == 0 }

    /// Human-readable display string, e.g. "⌃⌥⌘F"
    func displayString() -> String {
        guard !isEmpty else { return "Not set" }
        var parts: [String] = []
        if (carbonModifiers & UInt32(controlKey)) != 0 { parts.append("⌃") }
        if (carbonModifiers & UInt32(optionKey)) != 0 { parts.append("⌥") }
        if (carbonModifiers & UInt32(shiftKey)) != 0 { parts.append("⇧") }
        if (carbonModifiers & UInt32(cmdKey)) != 0 { parts.append("⌘") }

        if let name = Hotkey.keyName(for: keyCode) {
            parts.append(name)
        } else {
            parts.append("(\(keyCode))")
        }
        return parts.joined()
    }

    /// Maps common key codes to readable names.
    private static func keyName(for code: UInt16) -> String? {
        switch Int(code) {
        case kVK_ANSI_A: return "A"
        case kVK_ANSI_B: return "B"
        case kVK_ANSI_C: return "C"
        case kVK_ANSI_D: return "D"
        case kVK_ANSI_E: return "E"
        case kVK_ANSI_F: return "F"
        case kVK_ANSI_G: return "G"
        case kVK_ANSI_H: return "H"
        case kVK_ANSI_I: return "I"
        case kVK_ANSI_J: return "J"
        case kVK_ANSI_K: return "K"
        case kVK_ANSI_L: return "L"
        case kVK_ANSI_M: return "M"
        case kVK_ANSI_N: return "N"
        case kVK_ANSI_O: return "O"
        case kVK_ANSI_P: return "P"
        case kVK_ANSI_Q: return "Q"
        case kVK_ANSI_R: return "R"
        case kVK_ANSI_S: return "S"
        case kVK_ANSI_T: return "T"
        case kVK_ANSI_U: return "U"
        case kVK_ANSI_V: return "V"
        case kVK_ANSI_W: return "W"
        case kVK_ANSI_X: return "X"
        case kVK_ANSI_Y: return "Y"
        case kVK_ANSI_Z: return "Z"
        case kVK_ANSI_0: return "0"
        case kVK_ANSI_1: return "1"
        case kVK_ANSI_2: return "2"
        case kVK_ANSI_3: return "3"
        case kVK_ANSI_4: return "4"
        case kVK_ANSI_5: return "5"
        case kVK_ANSI_6: return "6"
        case kVK_ANSI_7: return "7"
        case kVK_ANSI_8: return "8"
        case kVK_ANSI_9: return "9"
        case kVK_Space: return "Space"
        case kVK_Return: return "↩"
        case kVK_Tab: return "⇥"
        case kVK_Delete: return "⌫"
        case kVK_ForwardDelete: return "⌦"
        case kVK_LeftArrow: return "←"
        case kVK_RightArrow: return "→"
        case kVK_UpArrow: return "↑"
        case kVK_DownArrow: return "↓"
        case kVK_F1: return "F1"
        case kVK_F2: return "F2"
        case kVK_F3: return "F3"
        case kVK_F4: return "F4"
        case kVK_F5: return "F5"
        case kVK_F6: return "F6"
        case kVK_F7: return "F7"
        case kVK_F8: return "F8"
        case kVK_F9: return "F9"
        case kVK_F10: return "F10"
        case kVK_F11: return "F11"
        case kVK_F12: return "F12"
        default: return nil
        }
    }
}

// MARK: - Time Formatting

extension Int {
    /// Formats as "HH:MM:SS"
    func asHMS() -> String {
        let h = self / 3600
        let m = (self % 3600) / 60
        let s = self % 60
        return String(format: "%02d:%02d:%02d", h, m, s)
    }

    /// Formats as "MM:SS"
    func asMS() -> String {
        let m = self / 60
        let s = self % 60
        return String(format: "%02d:%02d", m, s)
    }

    /// Adaptive format — shows hours only when needed
    func asAdaptiveTime() -> String {
        self >= 3600 ? asHMS() : asMS()
    }
}

// MARK: - Session State

enum SessionState: Equatable {
    case idle
    case running
    case interrupted
}
