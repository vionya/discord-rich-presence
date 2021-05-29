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

#[allow(dead_code)]
pub struct DiscordIpcClient {
    pub connected: bool,
    pub socket: Option<UnixStream>,
}

impl DiscordIpc for DiscordIpcClient {
    fn get_valid_path(&mut self) -> Result<Option<PathBuf>> {
        let mut path: Option<PathBuf> = None;

        for i in 0..10 {
            path = Some(DiscordIpcClient::get_pipe_pattern().join(format!("discord-ipc-{}", i)));

            if !path.as_ref().unwrap().exists() {
                continue;
            } else {
                break;
            }
        }

        Ok(path)
    }

    fn connect_ipc(&mut self) -> Result<()> {
        let path = self.get_valid_path()?.unwrap();
        let socket = UnixStream::connect(path).expect("Failed to open socket");
        self.socket = Some(socket);

        Ok(())
    }

    fn write(&mut self, data: &[u8]) -> Result<()> {
        let socket = self.socket.as_mut().unwrap();

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
        match socket.shutdown(Shutdown::Both) {
            Ok(()) => (),
            Err(_err) => (),
        };

        Ok(())
    }
}

impl DiscordIpcClient {
    fn get_pipe_pattern() -> PathBuf {
        let mut path = String::new();

        for key in &ENV_KEYS {
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
