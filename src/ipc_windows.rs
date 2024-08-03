use crate::discord_ipc::DiscordIpc;
use crate::Error;
use serde_json::json;
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    os::windows::fs::OpenOptionsExt,
    path::PathBuf,
};

type Result<T> = std::result::Result<T, Error>;

#[allow(dead_code)]
#[derive(Debug)]
/// A wrapper struct for the functionality contained in the
/// underlying [`DiscordIpc`](trait@DiscordIpc) trait.
pub struct DiscordIpcClient {
    /// Client ID of the IPC client.
    pub client_id: String,
    connected: bool,
    socket: Option<File>,
}

impl DiscordIpcClient {
    /// Creates a new `DiscordIpcClient`.
    ///
    /// # Examples
    /// ```
    /// let ipc_client = DiscordIpcClient::new("<some client id>")?;
    /// ```
    pub fn new(client_id: &str) -> Result<Self> {
        let client = Self {
            client_id: client_id.to_string(),
            connected: false,
            socket: None,
        };

        Ok(client)
    }
}

impl DiscordIpc for DiscordIpcClient {
    fn connect_ipc(&mut self) -> Result<()> {
        for i in 0..10 {
            let path = PathBuf::from(format!(r"\\?\pipe\discord-ipc-{}", i));

            match OpenOptions::new().access_mode(0x3).open(&path) {
                Ok(handle) => {
                    self.socket = Some(handle);
                    return Ok(());
                }
                Err(_) => continue,
            }
        }

        Err(Error::IPCConnectionFailled)
    }

    fn write(&mut self, data: &[u8]) -> Result<()> {
        let socket = self.socket.as_mut().ok_or(Error::NotConnected)?;

        socket.write_all(data)?;

        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<()> {
        let socket = self.socket.as_mut().ok_or(Error::NotConnected)?;

        socket.read_exact(buffer)?;

        Ok(())
    }

    fn close(&mut self) -> Result<()> {
        let data = json!({});
        if self.send(data, 2).is_ok() {} // ?

        let socket = self.socket.as_mut().ok_or(Error::NotConnected)?;

        socket.flush()?;

        Ok(())
    }

    fn get_client_id(&self) -> &String {
        &self.client_id
    }
}
