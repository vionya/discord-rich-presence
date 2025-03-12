use crate::{
    activity::Activity,
    error::Error,
    pack_unpack::{pack, unpack},
    voice_settings::VoiceSettings,
};
use serde_json::{json, Map, Value};
use uuid::Uuid;

type Result<T> = std::result::Result<T, Error>;

/// A client that connects to and communicates with the Discord IPC.
///
/// Implemented via the [`DiscordIpcClient`](struct@crate::DiscordIpcClient) struct.
pub trait DiscordIpc {
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
    /// let mut client = discord_rich_presence::new_client("<some client id>")?;
    /// client.connect()?;
    /// ```
    fn connect(&mut self) -> Result<()> {
        self.connect_ipc()?;
        self.send_handshake()?;

        Ok(())
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
    /// let mut client = discord_rich_presence::new_client("<some client id>")?;
    /// client.connect()?;
    ///
    /// client.close()?;
    /// client.reconnect()?;
    /// ```
    fn reconnect(&mut self) -> Result<()> {
        self.close()?;
        self.connect_ipc()?;
        self.send_handshake()?;

        Ok(())
    }

    #[doc(hidden)]
    fn get_client_id(&self) -> &String;

    #[doc(hidden)]
    fn connect_ipc(&mut self) -> Result<()>;

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
    fn send_handshake(&mut self) -> Result<()> {
        self.send(
            json!({
                "v": 1,
                "client_id": self.get_client_id()
            }),
            0,
        )?;

        // TODO: Return an Err if the handshake is rejected
        self.recv()?;

        Ok(())
    }

    /// Sends JSON data to the Discord IPC.
    ///
    /// This method takes data (`serde_json::Value`) and
    /// an opcode as its parameters.
    ///
    /// # Errors
    /// Returns an `Err` variant if writing to the socket failed
    ///
    /// # Examples
    /// ```
    /// let payload = serde_json::json!({ "field": "value" });
    /// client.send(payload, 0)?;
    /// ```
    fn send(&mut self, data: Value, opcode: u8) -> Result<()> {
        let data_string = data.to_string();
        let header = pack(opcode.into(), data_string.len() as u32);

        self.write(&header)?;
        self.write(data_string.as_bytes())?;

        Ok(())
    }

    #[doc(hidden)]
    fn write(&mut self, data: &[u8]) -> Result<()>;

    /// Receives an opcode and JSON data from the Discord IPC.
    ///
    /// This method returns any data received from the IPC.
    /// It returns a tuple containing the opcode, and the JSON data.
    ///
    /// # Errors
    /// Returns an `Err` variant if reading the socket was
    /// unsuccessful.
    ///
    /// # Examples
    /// ```
    /// client.connect_ipc()?;
    /// client.send_handshake()?;
    ///
    /// println!("{:?}", client.recv()?);
    /// ```
    fn recv(&mut self) -> Result<(u32, Value)> {
        let mut header = [0; 8];

        self.read(&mut header)?;
        let (op, length) = unpack(header.to_vec())?;

        let mut data = vec![0u8; length as usize];
        self.read(&mut data)?;

        let response = String::from_utf8(data.to_vec()).map_err(|_| Error::RecvUtf8Response)?;
        let json_data =
            serde_json::from_str::<Value>(&response).map_err(|_| Error::JsonParseResponse)?;

        Ok((op, json_data))
    }

    #[doc(hidden)]
    fn read(&mut self, buffer: &mut [u8]) -> Result<()>;

    /// Sends a command to the Discord IPC.
    ///
    /// This sends a command to Discord, as described
    /// [here](https://discord.com/developers/docs/topics/rpc#commands-and-events).
    ///
    /// The return value is the "data" field from the response payload.
    fn command(&mut self, cmd: &str, args: Value) -> Result<Value> {
        let nonce = Uuid::new_v4().to_string();
        let data = json!({
            "cmd": cmd,
            "args": args,
            "nonce": nonce.clone(),
        });
        self.send(data, 1)?;
        let (opcode, value) = self.recv()?;
        log::debug!("DRPC {}: {} {:?}", cmd, opcode, value);

        let mut value_obj = value.as_object();
        let temp_map = Map::new();
        let mut v = value_obj.get_or_insert(&temp_map).clone();

        let e = v.get("evt").unwrap();

        if !e.is_null() {
            // Event response
            let e = e.as_str().unwrap();
            if e == "ERROR" {
                let mut d = v.remove("data").unwrap().as_object().unwrap().clone();
                let code = d.remove("code").unwrap().as_u64().unwrap() as usize;
                let message = d.remove("message").unwrap().as_str().unwrap().to_string();
                return Err(Error::CommandError(code.into(), message));
            }

            todo!("check for other types of events")
        } else {
            // Command response
            let nonce_val = v.remove("nonce").unwrap();
            let returned_nonce = nonce_val.as_str().unwrap();
            if nonce != returned_nonce {
                return Err(Error::NonceCommandMismatch);
            }

            Ok(v.remove("data").unwrap())
        }
    }

    /// Sets a Discord activity.
    ///
    /// This method is an abstraction of [`send`],
    /// wrapping it such that only an activity payload
    /// is required.
    ///
    /// [`send`]: #method.send
    ///
    /// # Errors
    /// Returns an `Err` variant if sending the payload failed.
    fn set_activity(&mut self, activity_payload: Activity) -> Result<()> {
        self.command(
            "SET_ACTIVITY",
            json!({
                "pid": std::process::id(),
                "activity": activity_payload
            }),
        )?;

        Ok(())
    }

    /// Works the same as as [`set_activity`] but clears activity instead.
    ///
    /// [`set_activity`]: #method.set_activity
    ///
    /// # Errors
    /// Returns an `Err` variant if sending the payload failed.
    fn clear_activity(&mut self) -> Result<()> {
        self.command(
            "SET_ACTIVITY",
            json!({
                "pid": std::process::id(),
                "activity": None::<()>
            }),
        )?;

        Ok(())
    }

    /// Used to authorize the client, popping up a modal in-app for user authorization.
    ///
    /// Scopes must be from [this list](https://discord.com/developers/docs/topics/oauth2#shared-resources-oauth2-scopes).
    /// Returned value is an OAuth2 authorization code which can be used for an access token.
    ///
    /// Authorization and authentication is necessary for all scopes except setting activities.
    ///
    /// See [AUTHORIZE](https://discord.com/developers/docs/topics/rpc#authorize).
    fn authorize(&mut self, scopes: &[&str]) -> Result<String> {
        let args = json!({
            "client_id": self.get_client_id(),
            "scopes": scopes
        });
        let v = self.command("AUTHORIZE", args)?;

        Ok(v.get("code")
            .and_then(|c| c.as_str())
            .ok_or(Error::NoAuthorizationCode)?
            .to_string())
    }

    /// Used to authenticate the client. Access token is given by the standard OAuth2 token process.
    ///
    /// See [AUTHENTICATE](https://discord.com/developers/docs/topics/rpc#authenticate).
    fn authenticate(&mut self, access_token: &str) -> Result<()> {
        let args = json!({ "access_token": access_token });
        let v = self.command("AUTHENTICATE", args)?;

        if v.as_object().unwrap().contains_key("code") {
            Err(Error::AuthenticationFailed)
        } else {
            Ok(())
        }
    }

    /// Gets the current voice settings of the client.
    ///
    /// See [GET_VOICE_SETTINGS](https://discord.com/developers/docs/topics/rpc#getvoicesettings).
    fn get_voice_settings(&mut self) -> Result<VoiceSettings> {
        let args = json!({});
        let d = self.command("GET_VOICE_SETTINGS", args)?;
        Ok(serde_json::from_value(d).map_err(|_| Error::JsonParseResponse)?)
    }

    /// Sets the current voice settings of the client. Returns the current complete state of voice settings.
    ///
    /// Only one RPC client may control these settings at a time. No two clients may have the "rpc.voice.write" scope at once.
    ///
    /// See [SET_VOICE_SETTINGS](https://discord.com/developers/docs/topics/rpc#setvoicesettings).
    fn set_voice_settings(&mut self, args: VoiceSettings) -> Result<VoiceSettings> {
        let args = json!(args);
        let d = self.command("SET_VOICE_SETTINGS", args)?;
        Ok(serde_json::from_value(d).map_err(|_| Error::JsonParseResponse)?)
    }

    /// Closes the Discord IPC connection. Implementation is dependent on platform.
    fn close(&mut self) -> Result<()>;
}
