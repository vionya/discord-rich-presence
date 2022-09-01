use discord_rich_presence::{DiscordIpc, DiscordIpcClient};
use std::error::Error;

#[test]
fn test_updating() -> Result<(), Box<dyn Error>> {

    let mut client = DiscordIpcClient::new("771124766517755954")?;
    client.connect()?;

    assert!(client.connected);

    client.reconnect()?;

    assert!(client.connected);

    client.close()?;

    assert!(!client.connected);

    Ok (())
}