use crate::{
    activity::Activity,
    cmd,
    error::DiscordError,
    util::{pack, unpack},
    Opcode, PlatformIpcImpl,
};
use serde_json::{json, Value};
use std::{error::Error, io};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[allow(dead_code)]
pub struct DiscordIpcClient {
    client_id: String,
    connected: bool,
    ipc: GenericIpcImpl,
}

/// A client that connects to and communicates with the Discord IPC.
///
/// Implemented via the [`DiscordIpcClient`](struct@crate::DiscordIpcClient) struct.
impl DiscordIpcClient {
    pub fn new(client_id: impl ToString) -> Self {
        Self {
            client_id: client_id.to_string(),
            connected: true,
            ipc: GenericIpcImpl {
                connection: PlatformIpcImpl::default(),
            },
        }
    }

    /// Returns a reference to the client id of this [`DiscordIpcClient`].
    pub fn get_client_id(&self) -> &String {
        &self.client_id
    }

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
    pub fn connect(&mut self) -> Result<Value> {
        self.ipc.connect_ipc()?;
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
    pub fn reconnect(&mut self) -> Result<Value> {
        self.disconnect()?;
        self.ipc.connect_ipc()?;
        self.send_handshake()
    }

    /// Disconnects from the Discord IPC. Implementation is dependent on platform.
    pub fn disconnect(&mut self) -> Result<()> {
        _ = self.ipc.send(json!({}), Opcode::Close);
        // Delegate to trait platform-specific implementation
        self.ipc.close()?;
        Ok(())
        // TODO: set connected to false
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
        self.ipc
            .send(
                json!({
                    "v": 1,
                    "client_id": self.get_client_id()
                }),
                Opcode::Handshake,
            )
            .map(|(val, _)| val)
    }

        self.send(
            json!({
                "v": 1,
                "client_id": self.get_client_id()
            }),
            Opcode::Handshake,
        )

    // ABSTRACTIONS

    /// Sets a Discord activity.
    ///
    /// # Errors
    /// Returns an `Err` variant if sending the payload failed.
    pub fn set_activity(&mut self, activity_payload: Activity) -> Result<Value> {
        self.ipc
            .send(
                cmd!(
                    SET_ACTIVITY,
                    {
                        "pid": std::process::id(),
                        "activity": activity_payload
                    }
                ),
                Opcode::Frame,
            )
            .map(|(val, _)| val)
    }

    /// Works the same as as [`set_activity`] but clears activity instead.
    ///
    /// [`set_activity`]: #method.set_activity
    ///
    /// # Errors
    /// Returns an `Err` variant if sending the payload failed.
    pub fn clear_activity(&mut self) -> Result<Value> {
        self.ipc
            .send(
                cmd!(
                    SET_ACTIVITY,
                    {
                        "pid": std::process::id(),
                        "activity": None::<()>
                    }
                ),
                Opcode::Frame,
            )
            .map(|(val, _)| val)
    }
}

struct GenericIpcImpl {
    pub(crate) connection: PlatformIpcImpl,
}

impl GenericIpcImpl {
    fn connect_ipc(&mut self) -> Result<()> {
        self.connection.connect_ipc()
    }

    fn close(&mut self) -> io::Result<()> {
        self.connection.close()
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
    fn send(&mut self, data: Value, opcode: Opcode) -> Result<(Value, Opcode)> {
        let data_string = data.to_string();
        let header = pack(opcode.into(), data_string.len() as u32);

        self.connection.write(&header)?;
        self.connection.write(data_string.as_bytes())?;

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
    fn recv(&mut self) -> Result<(Value, Opcode)> {
        let mut header = [0; 8];

        self.connection.read(&mut header)?;
        let (op, length) = unpack(header.to_vec())?;
        let opcode = Opcode::from(op);

        let mut data = vec![0u8; length as usize];
        self.connection.read(&mut data)?;

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
        Ok((json_data, opcode))
    }
}
