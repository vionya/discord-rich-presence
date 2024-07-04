#[derive(Debug, thiserror::Error)]
/// A collection of the possible error
pub enum Error {
    /// TODO
    #[error("Could not connect to Discord's IPC socket")]
    IPCConnectionFailled,

    /// Client is not connected
    #[error("Could not get the underlying socket, client probably not connected")]
    Socket,

    /// TODO
    #[error("Error while interacting with Discord's IPC")]
    IPCIO(#[from] std::io::Error),

    /// TODO
    #[error("{0}")] // Transparent doesn't work on String
    Deserialisation(String),

    /// TODO
    #[error(
        "Could not unpack Discord IPC's data, expected: len {expected} but found len {received}"
    )]
    Unpack {
        /// Expected data size
        expected: usize,

        /// Actual data size
        received: usize,
    },
}
