use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{self, BufReader, Read, Write},
    os::windows::fs::OpenOptionsExt,
    path::PathBuf,
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Default, Debug)]
pub(crate) struct PlatformIpcImpl {
    reader: Option<BufReader<File>>,
    writer: Option<File>,
}

impl PlatformIpcImpl {
    pub(crate) fn connect_ipc(&mut self) -> Result<()> {
        for i in 0..10 {
            let path = PathBuf::from(format!(r"\\?\pipe\discord-ipc-{}", i));

            if let Ok(handle) = OpenOptions::new().access_mode(0x3).open(&path) {
                self.reader = Some(BufReader::new(handle.try_clone()?));
                self.writer = Some(handle.try_clone()?);
                return Ok(());
            }
        }

        Err("Couldn't connect to the Discord IPC socket".into())
    }

    pub(crate) fn close(&mut self) -> io::Result<()> {
        let socket = self.writer.as_mut().unwrap();
        socket.flush()
    }

    pub(crate) fn write(&mut self, data: &[u8]) -> io::Result<()> {
        let socket = self.writer.as_mut().expect("Client not connected");
        socket.write_all(data)
    }

    pub(crate) fn read(&mut self, buffer: &mut [u8]) -> io::Result<()> {
        let socket = self.reader.as_mut().unwrap();
        socket.read_exact(buffer)
    }
}
