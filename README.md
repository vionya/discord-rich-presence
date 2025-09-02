# Discord Rich Presence
[![crates.io](https://img.shields.io/crates/v/discord-rich-presence.svg)](https://crates.io/crates/discord-rich-presence)
[![Docs](https://docs.rs/discord-rich-presence/badge.svg?version=1.0.0)](https://docs.rs/discord-rich-presence)


A simple, cross-platform crate to connect and send data to Discord's IPC. Special attention is given to sending rich presence data.

## Example
```rust
use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DiscordIpcClient::new("<some application ID>");

    client.connect()?;
    client.set_activity(activity::Activity::new()
        .state("foo")
        .details("bar")
    )?;
    client.close()?;

    Ok(())
}
```

### Running example on the CLI
The repository comes with an example you can run with cargo to set a dummy activity. Just provide a valid Client ID as the argument:

```
cargo run --example presence <CLIENT_ID>
```
