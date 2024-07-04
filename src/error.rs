#[derive(Debug, thiserror::Error)]
/// A collection of the possible error
pub enum Error {
    /// Could not create the connection
    #[error("Could not connect to Discord's IPC socket")]
    IPCConnectionFailled,

    /// Underlying socket unreachable
    #[error("Could not get the underlying socket, client is probably not connected")]
    NotConnected,

    /// IO error while using the socket
    #[error("Error while interacting with Discord's IPC")]
    IPCIO(#[from] std::io::Error),

    /// Error while parsing data
    #[error("{0}")] // Transparent doesn't work on String
    Deserialisation(String),

    /// TODO
    #[error(
        "Could not unpack Discord IPC's data, expected {expected} bytes but found {received} bytes"
    )]
    Unpack {
        /// Expected data size
        expected: usize,

        /// Actual data size
        received: usize,
    },
}
