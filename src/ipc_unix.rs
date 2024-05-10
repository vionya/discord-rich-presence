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

const APP_SUBPATHS: [&str; 4] = [
    "",
    "app/com.discordapp.Discord/",
    "snap.discord-canary/",
    "snap.discord/",
];

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Default)]
pub(crate) struct PlatformIpcImpl {
    socket: Option<UnixStream>,
}

impl PlatformIpcImpl {
    fn get_pipe_pattern() -> PathBuf {
        let mut path = String::new();

        for key in &ENV_KEYS {
            if let Ok(val) = var(key) {
                path = val;
                break;
            }
        }
        PathBuf::from(path)
    }

    pub(crate) fn connect_ipc(&mut self) -> Result<()> {
        for i in 0..10 {
            for subpath in APP_SUBPATHS {
                let path = Self::get_pipe_pattern()
                    .join(subpath)
                    .join(format!("discord-ipc-{}", i));

                if let Ok(socket) = UnixStream::connect(&path) {
                    self.socket = Some(socket);
                    return Ok(());
                }
            }
        }

        Err("Couldn't connect to the Discord IPC socket".into())
    }

    pub(crate) fn write(&mut self, data: &[u8]) -> io::Result<()> {
        let socket = self.socket.as_mut().expect("Client not connected");
        socket.write_all(data)
    }

    pub(crate) fn read(&mut self, buffer: &mut [u8]) -> io::Result<()> {
        let socket = self.socket.as_mut().unwrap();
        socket.read_exact(buffer)
    }

    pub(crate) fn close(&mut self) -> io::Result<()> {
        let socket = self.socket.as_mut().unwrap();
        socket.flush()?;
        // Shutdown, but we don't care about if it's successful or not
        match socket.shutdown(Shutdown::Both) {
            Ok(()) => (),
            Err(_err) => (),
        };

        Ok(())
    }
}
