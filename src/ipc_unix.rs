use crate::discord_ipc::DiscordIpc;
use serde_json::json;
use std::os::unix::net::UnixStream;
use std::{
    env::var,
    error::Error,
    io::{Read, Write},
    net::Shutdown,
    path::PathBuf,
};

// Environment keys to search for the Discord pipe
const ENV_KEYS: [&str; 4] = ["XDG_RUNTIME_DIR", "TMPDIR", "TMP", "TEMP"];

type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// A wrapper struct for the functionality contained in the
/// underlying [`DiscordIpc`](trait@DiscordIpc) trait.
pub struct DiscordIpcClient {
    /// Client ID of the IPC client.
    pub client_id: String,
    socket: Option<UnixStream>,
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

    fn get_pipe_pattern() -> PathBuf {
        let mut path = String::new();

        for key in &ENV_KEYS {
            if let Ok(val) = var(key) {
                path = val
            }
        }

        PathBuf::from(path)
    }
}

impl DiscordIpc for DiscordIpcClient {
    fn connect_ipc(&mut self, ipc: Option<u8>) -> Result<()> {
        if let Some(ipc) = ipc {
            let path = DiscordIpcClient::get_pipe_pattern().join(format!("discord-ipc-{ipc}"));

            let Ok(socket) = UnixStream::connect(path) else {
                return Err("Selected socket doesnt exist".into());
            };

            self.socket = Some(socket);
            return Ok(());
        }

        for i in 0..10 {
            let path = DiscordIpcClient::get_pipe_pattern().join(format!("discord-ipc-{i}"));

            if let Ok(socket) = UnixStream::connect(path) {
                self.socket = Some(socket);
                return Ok(());
            }
        }

        Err("Couldn't connect to the Discord IPC socket".into())
    }

    fn write(&mut self, data: &[u8]) -> Result<()> {
        let Some(socket) = &mut self.socket else {
            return Err("Not connected to socket".into());
        };

        socket.write_all(data)?;

        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<()> {
        let Some(socket) = &mut self.socket else {
            return Err("Not connected to socket".into());
        };

        socket.read_exact(buffer)?;

        Ok(())
    }

    fn close(&mut self) -> Result<()> {
        let data = json!({});
        if self.send(data, 2).is_ok() {}

        let socket = self.socket.as_mut().unwrap();

        socket.flush()?;
        match socket.shutdown(Shutdown::Both) {
            Ok(()) => (),
            Err(_err) => (),
        };

        Ok(())
    }

    fn get_client_id(&self) -> &String {
        &self.client_id
    }
}
