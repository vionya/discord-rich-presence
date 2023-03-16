//! When something goes wrong when interacting with the Discord IPC.

/// Represents an error that occurs when using the Discord IPC
pub struct DiscordError {
    pub(crate) code: u64,
    pub(crate) message: String,
}

impl std::fmt::Display for DiscordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error (code {}): {}", self.code, self.message)
    }
}

impl std::fmt::Debug for DiscordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DiscordError")
            .field("code", &self.code)
            .field("message", &self.message)
            .finish()
    }
}

impl std::error::Error for DiscordError {}