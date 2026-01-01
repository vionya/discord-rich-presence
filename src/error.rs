//! Error types for this crate.
use thiserror::Error;

/// Error type for this crate.
#[derive(Error, Debug)]
pub enum Error {
    /// Failed to decode opcode.
    #[error("failed to decode opcode")]
    DecodeOpcode,
    /// Failed to decode header.
    #[error("failed to decode header")]
    DecodeHeader,

    /// Failed to recieve UTF-8 response.
    #[error("failed to recieve valid UTF-8 response")]
    RecvUtf8Response,
    /// Failed to parse response json.
    #[error("failed to parse response json")]
    JsonParseResponse,

    /// Failed to find IPC socket.
    #[error("failed to find IPC socket")]
    IPCNotFound,
    /// Failed to connect to IPC socket.
    #[error("failed to connect to IPC socket")]
    IPCConnectionFailed,
    /// Not connected to IPC socket.
    #[error("not connected to IPC socket")]
    NotConnected,

    /// Failed to read from IPC socket.
    #[error("failed to read to IPC socket")]
    ReadError(std::io::Error),
    /// Failed to write to IPC socket.
    #[error("failed to write to IPC socket")]
    WriteError(std::io::Error),
    /// Failed to flush IPC socket.
    #[error("failed to flush IPC socket")]
    FlushError(std::io::Error),
}
