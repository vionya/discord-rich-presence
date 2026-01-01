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

    fn find_pipe() -> Option<PathBuf> {
        let snap_flag = var("SNAP").is_ok();
        log::debug!("find_pipe: snap_flag is {}", snap_flag);

        for key in &ENV_KEYS {
            let base_path = match var(key) {
                Ok(val) => {
                    if snap_flag && key == &ENV_KEYS[0] {
                        let path = val.rsplit_once('/').map_or("", |(parent, _)| parent);
                        PathBuf::from(path)
                    } else {
                        PathBuf::from(val)
                    }
                },
                Err(_) => continue,
            };

            if !base_path.is_dir() { continue }

            for i in 0..10 {
                let pipe_name = format!("discord-ipc-{}", i);

                for subpath in APP_SUBPATHS {
                    let pipe_path = base_path
                        .join(subpath)
                        .join(&pipe_name);

                    if pipe_path.exists() {
                        log::debug!("find_pipe: found pipe at {}", pipe_path.display());
                        return Some(pipe_path)
                    }
                }
            }
        }

        log::warn!("find_pipe: could not find pipe");
        None
    }
}

impl DiscordIpc for DiscordIpcClient {
    fn connect_ipc(&mut self) -> Result<()> {
        let pipe = Self::find_pipe().ok_or(Error::IPCNotFound)?;

        match UnixStream::connect(&pipe) {
            Ok(socket) => {
                self.socket = Some(socket);
                return Ok(());
            }
            Err(err) => log::debug!("connect_ipc: {}", err),
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

#[cfg(test)]
mod tests {
    use std::env::{remove_var, set_var};
    use std::fs;

    use super::*;

    #[test]
    fn find_pipe() {
        remove_var("SNAP");
        set_var("XDG_RUNTIME_DIR", "/this/path/should/not/exist");
        set_var("TMPDIR", "/tmp");
        fs::write("/tmp/discord-ipc-4", "hi").expect("Could not create dummy pipe");

        assert_eq! {
            DiscordIpcClient::find_pipe(),
            Some(PathBuf::from("/tmp/discord-ipc-4"))
        };

        // FIXME: Snap needs better tests
        set_var("SNAP", "/snap/discord");

        assert_eq! {
            DiscordIpcClient::find_pipe(),
            Some(PathBuf::from("/tmp/discord-ipc-4"))
        };

        remove_var("XDG_RUNTIME_DIR");
        remove_var("TMPDIR");
        remove_var("TEMP");
        remove_var("TMP");

        assert_eq! {
            DiscordIpcClient::find_pipe(),
            None
        };

        set_var("XDG_RUNTIME_DIR", "/this/path/should/not/exist");

        assert_eq! {
            DiscordIpcClient::find_pipe(),
            None
        };

        fs::remove_file("/tmp/discord-ipc-4").expect("Could not remove dummy pipe");
    }
}
