use crate::{discord_ipc::DiscordIpc, Opcode};
use serde_json::json;
use std::os::unix::net::UnixStream;
use std::{
    env::var,
    error::Error,
    io::{self, Read, Write},
    net::Shutdown,
    path::PathBuf,
};

// Environment keys to search for the Discord pipe
const ENV_KEYS: [&str; 4] = ["XDG_RUNTIME_DIR", "TMPDIR", "TMP", "TEMP"];

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[allow(dead_code)]
/// A wrapper struct for the functionality contained in the
/// underlying [`DiscordIpc`](trait@DiscordIpc) trait.
pub struct DiscordIpcClient {
    /// Client ID of the IPC client.
    pub client_id: String,
    connected: bool,
    socket: Option<UnixStream>,
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

    fn get_pipe_pattern() -> PathBuf {
        let mut path = String::new();

        for key in &ENV_KEYS {
            // TODO: Refactor to if let
            match var(key) {
                Ok(val) => {
                    path = val;
                    break;
                }
                Err(_e) => continue,
            }
        }
        PathBuf::from(path)
    }
}

impl DiscordIpc for DiscordIpcClient {
    fn connect_ipc(&mut self) -> Result<()> {
        for i in 0..10 {
            let path = DiscordIpcClient::get_pipe_pattern().join(format!("discord-ipc-{}", i));

            if let Ok(socket) = UnixStream::connect(&path) {
                self.socket = Some(socket);
                return Ok(());
            }
        }

        Err("Couldn't connect to the Discord IPC socket".into())
    }

    fn write(&mut self, data: &[u8]) -> io::Result<()> {
        let socket = self.socket.as_mut().expect("Client not connected");
        socket.write_all(data)
    }

    fn read(&mut self, buffer: &mut [u8]) -> io::Result<()> {
        let socket = self.socket.as_mut().unwrap();
        socket.read_exact(buffer)
    }

    fn close(&mut self) -> io::Result<()> {
        let data = json!({});
        if self.send(data, Opcode::Close).is_ok() {}

        let socket = self.socket.as_mut().unwrap();
        socket.flush()?;
        // Shutdown, but we don't care about if it's successful or not
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
