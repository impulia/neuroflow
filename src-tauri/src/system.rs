pub fn get_idle_time() -> f64 {
    #[cfg(target_os = "macos")]
    {
        #[link(name = "CoreGraphics", kind = "framework")]
        extern "C" {
            fn CGEventSourceSecondsSinceLastEventType(state: i32, event_type: u32) -> f64;
        }
        // kCGEventSourceStateCombinedSessionState = 0
        // kCGAnyInputEventType = u32::MAX
        unsafe { CGEventSourceSecondsSinceLastEventType(0, u32::MAX) }
    }
    #[cfg(not(target_os = "macos"))]
    {
        // Fallback for non-macOS systems (e.g. for development/testing on Linux)
        0.0
    }
}
