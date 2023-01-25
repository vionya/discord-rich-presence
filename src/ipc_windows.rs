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

#[allow(dead_code)]
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
    pub fn new(client_id: &str) -> Self {
        Self {
            client_id: client_id.to_string(),
            connected: false,
            socket: None,
        };
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

        Err("Couldn't connect to the Discord IPC socket".into())
    }

    fn write(&mut self, data: &[u8]) -> Result<()> {
        let socket = self.socket.as_mut().expect("Client not connected");

        socket.write_all(data)?;

        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<()> {
        let socket = self.socket.as_mut().unwrap();

        socket.read_exact(buffer)?;

        Ok(())
    }

    fn close(&mut self) -> Result<()> {
        let data = json!({});
        if self.send(data, 2).is_ok() {}

        let socket = self.socket.as_mut().unwrap();
        socket.flush()?;

        Ok(())
    }

    fn get_client_id(&self) -> &String {
        &self.client_id
    }
}
