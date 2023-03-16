use crate::{
    activity::models::Activity,
    error::DiscordError,
    util::{pack, unpack},
    Opcode,
};
use serde_json::{json, Value};
use std::{error::Error, io};
use uuid::Uuid;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// A client that connects to and communicates with the Discord IPC.
///
/// Implemented via the [`DiscordIpcClient`](struct@crate::DiscordIpcClient) struct.
pub trait DiscordIpc {
    #[doc(hidden)]
    fn get_client_id(&self) -> &String;

    #[doc(hidden)]
    fn connect_ipc(&mut self) -> Result<()>;

    /// Closes the Discord IPC connection. Implementation is dependent on platform.
    fn close(&mut self) -> io::Result<()>;

    #[doc(hidden)]
    fn write(&mut self, data: &[u8]) -> io::Result<()>;

    #[doc(hidden)]
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<()>;

    /// Connects the client to the Discord IPC.
    ///
    /// This method attempts to first establish a connection,
    /// and then sends a handshake.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant if the client
    /// fails to connect to the socket, or if it fails to
    /// send a handshake.
    ///
    /// # Examples
    /// ```
    /// let mut client = DiscordIpcClient::new("<some client id>");
    /// client.connect()?;
    /// ```
    fn connect(&mut self) -> Result<Value> {
        self.connect_ipc()?;
        self.send_handshake()
    }

    /// Reconnects to the Discord IPC.
    ///
    /// This method closes the client's active connection,
    /// then re-connects it and re-sends a handshake.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant if the client
    /// failed to connect to the socket, or if it failed to
    /// send a handshake.
    ///
    /// # Examples
    /// ```
    /// let mut client = DiscordIpcClient::new("<some client id>");
    /// client.connect()?;
    ///
    /// client.close()?;
    /// client.reconnect()?;
    /// ```
    fn reconnect(&mut self) -> Result<Value> {
        self.close()?;
        self.connect_ipc()?;
        self.send_handshake()
    }

    /// Handshakes the Discord IPC.
    ///
    /// This method sends the handshake signal to the IPC.
    /// It is usually not called manually, as it is automatically
    /// called by [`connect`] and/or [`reconnect`].
    ///
    /// [`connect`]: #method.connect
    /// [`reconnect`]: #method.reconnect
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant if sending the handshake failed.
    #[doc(hidden)]
    fn send_handshake(&mut self) -> Result<Value> {
        self.send(
            json!({
                "v": 1,
                "client_id": self.get_client_id()
            }),
            Opcode::Handshake,
        )
    }

    /// Sends JSON data to the Discord IPC.
    ///
    /// This method takes [data](`serde_json::Value`) and
    /// an [opcode](`Opcode`) as its parameters.
    ///
    /// # Errors
    /// Returns an `Err` variant if writing to the socket failed
    ///
    /// # Examples
    /// ```
    /// let payload = serde_json::json!({ "field": "value" });
    /// client.send(payload, Opcode::Handshake)?;
    /// ```
    fn send(&mut self, data: Value, opcode: Opcode) -> Result<Value> {
        let data_string = data.to_string();
        let header = pack(opcode.into(), data_string.len() as u32);

        self.write(&header)?;
        self.write(data_string.as_bytes())?;

        self.recv()
    }

    /// Receives an opcode and JSON data from the Discord IPC.
    ///
    /// This method returns any data received from the IPC.
    /// It returns a tuple containing the opcode, and the JSON data.
    ///
    /// # Errors
    /// Returns an `Err` variant if reading the socket was unsuccessful.
    ///
    /// # Examples
    /// ```
    /// client.connect_ipc()?;
    /// client.send_handshake()?;
    ///
    /// println!("{:?}", client.recv()?);
    /// ```
    #[doc(hidden)]
    fn recv(&mut self) -> Result<Value> {
        let mut header = [0; 8];

        self.read(&mut header)?;
        let (op, length) = unpack(header.to_vec())?;
        let opcode = Opcode::from(op);

        let mut data = vec![0u8; length as usize];
        self.read(&mut data)?;

        let response = String::from_utf8(data.to_vec())?;
        let json_data = serde_json::from_str::<Value>(&response)?;

        // ERROR HANDLING
        let evt = &json_data["evt"];
        // If the opcode is Close, then the response body is different from a standard
        // payload, so it needs to be handled separately
        if opcode == Opcode::Close {
            // For a Close response, the data is at the top level
            let (code, msg) = (&json_data["code"], &json_data["message"]);
            if code.is_u64() && msg.is_string() {
                return Err(DiscordError {
                    code: code.as_u64().unwrap(),
                    message: msg.as_str().unwrap().to_string(),
                }
                .into());
            }
        // If the "evt" key is "ERROR", then there's an error to return
        } else if evt.is_string() && evt.as_str().unwrap() == "ERROR" {
            // For any other response opcode, the data is nested, since the response is
            // the sent command echoed
            let (code, msg) = (&json_data["data"]["code"], &json_data["data"]["message"]);
            if code.is_u64() && msg.is_string() {
                return Err(DiscordError {
                    code: code.as_u64().unwrap(),
                    message: msg.as_str().unwrap().to_string(),
                }
                .into());
            }
        };
        Ok(json_data)
    }

    /// Sets a Discord activity.
    ///
    /// This method is an abstraction of [`send`], wrapping it such that only an
    /// activity payload is required.
    ///
    /// [`send`]: #method.send
    ///
    /// # Errors
    /// Returns an `Err` variant if sending the payload failed.
    fn set_activity(&mut self, activity_payload: Activity) -> Result<Value> {
        let data = json!({
            "cmd": "SET_ACTIVITY",
            "args": {
                "pid": std::process::id(),
                "activity": activity_payload
            },
            "nonce": Uuid::new_v4().to_string()
        });
        self.send(data, Opcode::Frame)
    }

    /// Works the same as as [`set_activity`] but clears activity instead.
    ///
    /// [`set_activity`]: #method.set_activity
    ///
    /// # Errors
    /// Returns an `Err` variant if sending the payload failed.
    fn clear_activity(&mut self) -> Result<Value> {
        let data = json!({
            "cmd": "SET_ACTIVITY",
            "args": {
                "pid": std::process::id(),
                "activity": None::<()>
            },
            "nonce": Uuid::new_v4().to_string()
        });
        self.send(data, Opcode::Frame)
    }
}
