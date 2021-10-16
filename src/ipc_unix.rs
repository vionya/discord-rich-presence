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
    pub client_id: String,
    pub connected: bool,
    pub socket: Option<UnixStream>,
}

impl DiscordIpc for DiscordIpcClient {
    fn connect_ipc(&mut self) -> Result<()> {
        for i in 0..10 {
            let path = DiscordIpcClient::get_pipe_pattern().join(format!("discord-ipc-{}", i));

            match UnixStream::connect(&path) {
                Ok(socket) => {
                    self.socket = Some(socket);
                    return Ok(());
                }
                Err(_) => continue,
            }
        }

        Err("Couldn't connect to the Discord IPC socket".into())
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

    fn get_client_id(&self) -> &String {
        &self.client_id
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
