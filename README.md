# Discord Rich Presence
A simple, cross-platform crate to connect and send data to Discord's IPC. Special attention is given to sending rich presence data.

## Example
```rust
use discord_rich_presence::{new_client, DiscordIpc};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = new_client("<some application ID>")?;

    client.set_activity(json!({
        "state": "foo",
        "details": "bar"
    }))?;

    client.close()?;
    Ok(())
}
```