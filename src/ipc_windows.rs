use crate::{discord_ipc::DiscordIpc, Opcode};
use serde_json::json;
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{self, Read, Write},
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
    /// let ipc_client = DiscordIpcClient::new("<some client id>");
    /// ```
    pub fn new(client_id: &str) -> Self {
        Self {
            client_id: client_id.to_string(),
            connected: false,
            socket: None,
        }
    }
}

impl DiscordIpc for DiscordIpcClient {
    fn get_client_id(&self) -> &String {
        &self.client_id
    }

    fn connect_ipc(&mut self) -> Result<()> {
        for i in 0..10 {
            let path = PathBuf::from(format!(r"\\?\pipe\discord-ipc-{}", i));

            if let Ok(handle) = OpenOptions::new().access_mode(0x3).open(&path) {
                self.socket = Some(handle);
                return Ok(());
            }
        }

        Err("Couldn't connect to the Discord IPC socket".into())
    }

    fn close(&mut self) -> io::Result<()> {
        _ = self.send(json!({}), Opcode::Close);
        let socket = self.socket.as_mut().unwrap();
        socket.flush()
    }

    fn write(&mut self, data: &[u8]) -> io::Result<()> {
        let socket = self.socket.as_mut().expect("Client not connected");
        socket.write_all(data)
    }

    fn read(&mut self, buffer: &mut [u8]) -> io::Result<()> {
        let socket = self.socket.as_mut().unwrap();
        socket.read_exact(buffer)
    }
}
