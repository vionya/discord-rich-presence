# Discord Rich Presence
[![crates.io](https://img.shields.io/crates/v/discord-rich-presence.svg)](https://crates.io/crates/discord-rich-presence)
[![Docs](https://docs.rs/discord-rich-presence/badge.svg?version=0.1.5)](https://docs.rs/discord-rich-presence)


A simple, cross-platform crate to connect and send data to Discord's IPC. Special attention is given to sending rich presence data.

## Example
```rust
use discord_rich_presence::{activity, new_client, DiscordIpc};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = new_client("<some application ID>")?;

    client.connect()?;
    client.set_activity(activity::Activity::new()
        .state("foo")
        .details("bar")
    )?;
    client.close()?;

    Ok(())
}
```