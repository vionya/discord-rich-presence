use crate::discord_ipc::DiscordIpc;
use serde_json::json;
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{Read, Write},
    os::windows::fs::OpenOptionsExt,
    path::PathBuf,
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// A wrapper struct for the functionality contained in the
/// underlying [`DiscordIpc`](trait@DiscordIpc) trait.
pub struct DiscordIpcClient {
    /// Client ID of the IPC client.
    pub client_id: String,
    socket: Option<File>,
}

impl DiscordIpcClient {
    /// Creates a new `DiscordIpcClient`.
    ///
    /// # Examples
    /// ```
    /// let ipc_client = DiscordIpcClient::new("<some client id>")?;
    /// ```
    pub fn new(client_id: &str) -> Self {
        Self {
            client_id: client_id.to_string(),
            socket: None,
        }
    }
}

impl DiscordIpc for DiscordIpcClient {
    fn connect_ipc(&mut self, ipc: Option<u8>) -> Result<()> {
        if let Some(ipc) = ipc {
            let path = PathBuf::from(format!(r"\\?\pipe\discord-ipc-{ipc}"));

            let Ok(socket) = OpenOptions::new().access_mode(0x3).open(&path) else {
                return Err("Selected socket not found".into())
            };

            self.socket = Some(socket);
            return Ok(());
        }

        for i in 0..10 {
            let path = PathBuf::from(format!(r"\\?\pipe\discord-ipc-{}", i));

            if let Ok(socket) = OpenOptions::new().access_mode(0x3).open(&path) {
                self.socket = Some(socket);
                return Ok(());
            }
        }

        Err("Couldn't connect to the Discord IPC socket".into())
    }

    fn write(&mut self, data: &[u8]) -> Result<()> {
        let Some(socket) = &mut self.socket else {
            return Err("Client not connected".into());
        };

        socket.write_all(data)?;

        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<()> {
        let Some(socket) = &mut self.socket else {
            return Err("Client not connected".into());
        };

        socket.read_exact(buffer)?;

        Ok(())
    }

    fn close(&mut self) -> Result<()> {
        let Some(socket) = &mut self.socket else {
            return Err("Client not connected".into());
        };
        socket.flush()?;

        Ok(())
    }

    fn get_client_id(&self) -> &String {
        &self.client_id
    }
}
