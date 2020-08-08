/// Escape whatever's being sent
/// in here to an ansi code
pub struct Ansi {}

impl Ansi {
    /// Add the required escape and terminator characters to
    /// an ansi code.
    pub fn escape(code: u8) -> String {
        format!("\x1B[{}m", code)
    }
}