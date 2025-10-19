use crate::{discord_ipc::DiscordIpc, error::Error};
use serde_json::json;
use std::{
    env::var,
    io::{Read, Write},
    net::Shutdown,
    os::unix::net::UnixStream,
    path::PathBuf,
};

// Environment keys to search for the Discord pipe
const ENV_KEYS: [&str; 4] = ["XDG_RUNTIME_DIR", "TMPDIR", "TMP", "TEMP"];

const APP_SUBPATHS: [&str; 7] = [
    "",
    "app/com.discordapp.Discord/",
    "app/dev.vencord.Vesktop/",
    ".flatpak/com.discordapp.Discord/xdg-run/",
    ".flatpak/dev.vencord.Vesktop/xdg-run/",
    "snap.discord-canary/",
    "snap.discord/",
];

type Result<T> = std::result::Result<T, Error>;

#[allow(dead_code)]
#[derive(Debug)]
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
    /// let ipc_client = DiscordIpcClient::new("<some client id>");
    /// ```
    pub fn new<T: AsRef<str>>(client_id: T) -> Self {
        Self {
            client_id: client_id.as_ref().to_string(),
            socket: None,
        }
    }

    fn get_pipe_pattern() -> PathBuf {
        log::debug!("get_pipe_pattern: {}", var("SNAP").is_ok());
        let mut path = String::new();

        for key in &ENV_KEYS {
            match var(key) {
                Ok(val) => {
                    if var("SNAP").is_ok() {
                        if key == &ENV_KEYS[0] {
                            path = val
                                .rsplit_once('/')
                                .map(|(parent, _)| parent)
                                .unwrap_or("")
                                .to_string();
                        }
                    } else {
                        path = val;
                    }
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
            for subpath in APP_SUBPATHS {
                let path = DiscordIpcClient::get_pipe_pattern()
                    .join(subpath)
                    .join(format!("discord-ipc-{}", i));

                log::debug!("connect_ipc: {}", path.display());

                match UnixStream::connect(&path) {
                    Ok(socket) => {
                        self.socket = Some(socket);
                        return Ok(());
                    }
                    Err(err) => {
                        log::debug!("connect_ipc: {}", err);
                        continue;
                    }
                }
            }
        }

        Err(Error::IPCConnectionFailed)
    }

    fn write(&mut self, data: &[u8]) -> Result<()> {
        let socket = self.socket.as_mut().ok_or(Error::NotConnected)?;

        socket.write_all(data).map_err(Error::WriteError)?;

        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<()> {
        let socket = self.socket.as_mut().ok_or(Error::NotConnected)?;

        socket.read_exact(buffer).map_err(Error::ReadError)?;

        Ok(())
    }

    fn close(&mut self) -> Result<()> {
        let data = json!({});
        if self.send(data, 2).is_ok() {}

        let socket = self.socket.as_mut().ok_or(Error::NotConnected)?;

        socket.flush().map_err(Error::FlushError)?;
        match socket.shutdown(Shutdown::Both) {
            Ok(()) => (),
            Err(_err) => (),
        };

        Ok(())
    }

    fn get_client_id(&self) -> &str {
        &self.client_id
    }
}
