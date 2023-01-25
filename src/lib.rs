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
//!     let mut client = DiscordIpcClient::new("<some client id>")?;
//!     client.connect(None)?;
//!
//!     let payload = activity::Activity::new().state("Hello world!");
//!     client.set_activity(payload)?;
//! }
//! ```
#![deny(missing_docs)]

mod discord_ipc;
mod pack_unpack;
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
