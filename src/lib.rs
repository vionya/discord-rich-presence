//! This library provides easy access to the Discord IPC.
//!
//! It provides implementations for both Unix and Windows
//! operating systems, with both implementations using the
//! same API. Thus, this crate can be used in a platform-agnostic
//! manner.
//!
//! # Hello world
//! ```
//! use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut client = DiscordIpcClient::new("<some client id>");
//!     client.connect()?;
//!
//!     let payload = activity::Activity::new().state("Hello world!");
//!     client.set_activity(payload)?;
//! }
//! ```
#![deny(missing_docs)]

mod discord_ipc;
pub mod error;
mod util;
pub use discord_ipc::*;
pub mod activity;

#[cfg(unix)]
mod ipc_unix;
#[cfg(unix)]
use ipc_unix as ipc;

#[cfg(windows)]
mod ipc_windows;
#[cfg(windows)]
use ipc_windows as ipc;

pub use ipc::DiscordIpcClient;

/// Models Discord's RPC opcodes for convenience
#[derive(Debug, PartialEq)]
pub enum Opcode {
    /// Handshake opcode for connecting to the IPC
    Handshake = 0,
    /// Frame opcode for sending commands to the IPC
    Frame = 1,
    /// Close opcode for closing the IPC connection
    Close = 2,
    /// Ping opcode for pinging the IPC
    Ping = 3,
    /// Pong opcode for ponging the IPC
    Pong = 4,
}

impl From<Opcode> for u32 {
    fn from(val: Opcode) -> Self {
        val as u32
    }
}

impl From<u32> for Opcode {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Handshake,
            1 => Self::Frame,
            2 => Self::Close,
            3 => Self::Ping,
            4 => Self::Pong,
            // Anything else is a bad value, so we let it be CLOSE
            _ => Self::Close,
        }
    }
}
