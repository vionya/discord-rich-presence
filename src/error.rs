//! Error types for this crate.
use core::fmt;

use thiserror::Error;

// TODO: add these error codes for the command method https://discord.com/developers/docs/topics/opcodes-and-status-codes#rpc

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

    /// Nonce command mismatch.
    #[error("nonce command mismatch")]
    NonceCommandMismatch,
    /// IPC Command Error.
    #[error("ipc command error ({0}): {1}")]
    CommandError(RPCErrorCode, String),

    // /// Failed to find data in response.
    // #[error("failed to find data in response")]
    // NoData,
    /// Failed to find authorization code in response.
    #[error("failed to find authorization code in response")]
    NoAuthorizationCode,
    /// Authentication failed.
    #[error("authentication failed")]
    AuthenticationFailed,
}

/// RPC Error Code
#[derive(Debug, Clone, Copy)]
#[repr(usize)]
pub enum RPCErrorCode {
    /// An undocumented error occurred.
    UndocumentedError = 0,
    /// An unknown error occurred.
    UnknownError = 1000,
    /// You sent an invalid payload.
    InvalidPayload = 4000,
    /// Invalid command name specified.
    InvalidCommand = 4002,
    /// Invalid guild ID specified.
    InvalidGuild = 4003,
    /// Invalid event name specified.
    InvalidEvent = 4004,
    /// Invalid channel ID specified.
    InvalidChannel = 4005,
    /// You lack permissions to access the given resource.
    InvalidPermissions = 4006,
    /// An invalid OAuth2 application ID was used to authorize or authenticate with.
    InvalidClientId = 4007,
    /// An invalid OAuth2 application origin was used to authorize or authenticate with.
    InvalidOrigin = 4008,
    /// An invalid OAuth2 token was used to authorize or authenticate with.
    InvalidToken = 4009,
    /// The specified user ID was invalid.
    InvalidUser = 4010,
    /// A standard OAuth2 error occurred; check the data object for the OAuth2 error details.
    OAuth2Error = 5000,
    /// An asynchronous `SELECT_TEXT_CHANNEL`/`SELECT_VOICE_CHANNEL` command timed out.
    SelectChannelTimedOut = 5001,
    /// An asynchronous `GET_GUILD` command timed out.
    GetGuildTimedOut = 5002,
    /// You tried to join a user to a voice channel but the user was already in one.
    SelectVoiceForceRequired = 5003,
    /// You tried to capture more than one shortcut key at once.
    CaptureShortcutAlreadyListening = 5004,
}
impl From<usize> for RPCErrorCode {
    fn from(x: usize) -> Self {
        match x {
            x if x == Self::UnknownError as usize => Self::UnknownError,
            x if x == Self::InvalidPayload as usize => Self::InvalidPayload,
            x if x == Self::InvalidCommand as usize => Self::InvalidCommand,
            x if x == Self::InvalidGuild as usize => Self::InvalidGuild,
            x if x == Self::InvalidEvent as usize => Self::InvalidEvent,
            x if x == Self::InvalidChannel as usize => Self::InvalidChannel,
            x if x == Self::InvalidPermissions as usize => Self::InvalidPermissions,
            x if x == Self::InvalidClientId as usize => Self::InvalidClientId,
            x if x == Self::InvalidOrigin as usize => Self::InvalidOrigin,
            x if x == Self::InvalidToken as usize => Self::InvalidToken,
            x if x == Self::InvalidUser as usize => Self::InvalidUser,
            x if x == Self::OAuth2Error as usize => Self::OAuth2Error,
            x if x == Self::SelectChannelTimedOut as usize => Self::SelectChannelTimedOut,
            x if x == Self::GetGuildTimedOut as usize => Self::GetGuildTimedOut,
            x if x == Self::SelectVoiceForceRequired as usize => Self::SelectVoiceForceRequired,
            x if x == Self::CaptureShortcutAlreadyListening as usize => {
                Self::CaptureShortcutAlreadyListening
            }
            _ => Self::UndocumentedError,
        }
    }
}
impl fmt::Display for RPCErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}({})", self, *self as usize)
    }
}

/// RPC Close Event Code
#[derive(Debug, Clone, Copy)]
#[repr(usize)]
pub enum RPCCloseEventCode {
    /// An undocumented error occurred.
    UndocumentedError = 0,
    /// You connected to the RPC server with an invalid client ID.
    InvalidClientId = 4000,
    /// You connected to the RPC server with an invalid origin.
    InvalidOrigin = 4001,
    /// You are being rate limited.
    RateLimited = 4002,
    /// The OAuth2 token associated with a connection was revoked, get a new one!
    TokenRevoked = 4003,
    /// The RPC Server version specified in the connection string was not valid.
    InvalidVersion = 4004,
    /// The encoding specified in the connection string was not valid.
    InvalidEncoding = 4005,
}
impl From<usize> for RPCCloseEventCode {
    fn from(x: usize) -> Self {
        match x {
            x if x == Self::InvalidClientId as usize => Self::InvalidClientId,
            x if x == Self::InvalidOrigin as usize => Self::InvalidOrigin,
            x if x == Self::RateLimited as usize => Self::RateLimited,
            x if x == Self::TokenRevoked as usize => Self::TokenRevoked,
            x if x == Self::InvalidVersion as usize => Self::InvalidVersion,
            x if x == Self::InvalidEncoding as usize => Self::InvalidEncoding,
            _ => Self::UndocumentedError,
        }
    }
}
impl fmt::Display for RPCCloseEventCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}({})", self, *self as usize)
    }
}
