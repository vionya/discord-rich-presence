mod discord_ipc;
mod pack_unpack;
pub use discord_ipc::*;

#[cfg(unix)]
mod ipc_unix;
#[cfg(unix)]
use ipc_unix as ipc;

#[cfg(windows)]
mod ipc_windows;
#[cfg(windows)]
use ipc_windows as ipc;

/// Creates a new client to connect to the Discord IPC.
/// This method automatically creates and connects a client.
pub fn new_client(client_id: &str) -> Result<impl DiscordIpc, Box<dyn std::error::Error>> {
    let mut client = ipc::DiscordIpcClient {
        connected: false,
        socket: None,
    };

    client.connect(client_id)?;
    Ok(client)
}
