use crate::pack_unpack::{pack, unpack};
use serde_json::{json, Value};
use std::error::Error;
use uuid::Uuid;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub trait DiscordIpc {
    fn connect(&mut self, client_id: String) -> Result<()> {
        self.connect_ipc()?;
        self.send_handshake(client_id)?;

        Ok(())
    }

    fn connect_ipc(&mut self) -> Result<()>;

    fn send_handshake(&mut self, client_id: String) -> Result<()> {
        self.send(
            json!({
                "v": 1,
                "client_id": client_id
            }),
            0,
        )?;
        self.recv()?;

        Ok(())
    }

    fn send(&mut self, data: serde_json::Value, opcode: u8) -> Result<()> {
        let data_string = data.to_string();
        let header = pack(opcode.into(), data_string.len() as u32)?;

        self.write(&header)?;
        self.write(data_string.as_bytes())?;

        Ok(())
    }

    fn write(&mut self, data: &[u8]) -> Result<()>;

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

    fn read(&mut self, buffer: &mut [u8]) -> Result<()>;

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

    fn close(&mut self) -> Result<()>;
}
