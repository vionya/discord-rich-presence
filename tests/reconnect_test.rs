use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use std::error::Error;

#[test]
fn test_reconnect() -> Result<(), Box<dyn Error>> {
    let mut client = DiscordIpcClient::new("771124766517755954");
    loop {
        if client.connect().is_ok() {
            break;
        }
    }

    loop {
        let payload = activity::ActivityBuilder::default()
            .state("part 1 (test)")
            .details("a placeholder")
            .assets(
                activity::AssetsBuilder::default()
                    .large_image("large-image")
                    .large_text("a thing")
                    .build(),
            )
            .build();

        if client.set_activity(payload).is_err() && client.reconnect().is_ok() {
            continue;
        }

        std::thread::sleep(std::time::Duration::from_secs(2));
    }

    #[allow(unreachable_code)]
    Ok(())
}
