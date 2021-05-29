use crate::pack_unpack::{pack, unpack};
use serde_json::{json, Value};
use std::error::Error;
use uuid::Uuid;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// A client that connects to and communicates with the Discord IPC.
pub trait DiscordIpc {
    /// Connects the client to the Discord IPC.
    /// This method is typically called automatically by the new_client function.
    fn connect(&mut self) -> Result<()> {
        self.connect_ipc()?;
        self.send_handshake()?;

        Ok(())
    }

    /// Reconnects to the Discord IPC.
    /// Active connections will be closed.
    fn reconnect(&mut self) -> Result<()> {
        self.close()?;
        self.connect_ipc()?;
        self.send_handshake()?;

        Ok(())
    }

    #[doc(hidden)]
    fn get_client_id(&self) -> &String;

    /// Tries to find a valid socket to connect to.
    fn get_valid_path(&mut self) -> Result<Option<std::path::PathBuf>>;

    #[doc(hidden)]
    fn connect_ipc(&mut self) -> Result<()>;

    /// Handshakes the Discord IPC. Usually called automatically by `connect`
    fn send_handshake(&mut self) -> Result<()> {
        self.send(
            json!({
                "v": 1,
                "client_id": self.get_client_id()
            }),
            0,
        )?;
        self.recv()?;

        Ok(())
    }

    /// Sends JSON data to the Discord IPC.
    fn send(&mut self, data: serde_json::Value, opcode: u8) -> Result<()> {
        let data_string = data.to_string();
        let header = pack(opcode.into(), data_string.len() as u32)?;

        self.write(&header)?;
        self.write(data_string.as_bytes())?;

        Ok(())
    }

    #[doc(hidden)]
    fn write(&mut self, data: &[u8]) -> Result<()>;

    /// Receives an opcode and JSON data from the Discord IPC.
    fn recv(&mut self) -> Result<(u32, Value)> {
        let mut header = [0; 8];

        self.read(&mut header)?;
        let (op, length) = unpack(header.to_vec())?;

        let mut data = vec![0u8; length as usize];
        self.read(&mut data)?;

        let response = String::from_utf8(data.to_vec())?;
        let json_data = serde_json::from_str::<Value>(&response)?;

        Ok((op, json_data))
    }

    #[doc(hidden)]
    fn read(&mut self, buffer: &mut [u8]) -> Result<()>;

    /// An abstraction for sending rich presence data to the IPC such that only the presence's JSON payload is required.
    fn set_activity(&mut self, activity_payload: Value) -> Result<()> {
        let data = json!({
            "cmd": "SET_ACTIVITY",
            "args": {
                "pid": std::process::id(),
                "activity": activity_payload
            },
            "nonce": Uuid::new_v4().to_string()
        });
        self.send(data, 1)?;

        Ok(())
    }

    /// Closes the Discord IPC connection. Implementation is dependent on platform.
    fn close(&mut self) -> Result<()>;
}
