use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use std::error::Error;

#[test]
fn test_updating() -> Result<(), Box<dyn Error>> {
    let mut client = DiscordIpcClient::new("771124766517755954");
    client.connect()?;

    client.set_activity(
        activity::ActivityBuilder::default()
            .state("part 1 (test)")
            .details("a placeholder")
            .assets(
                activity::AssetsBuilder::default()
                    .large_image("large-image")
                    .large_text("a thing")
                    .build(),
            )
            .build(),
    )?;
    std::thread::sleep(std::time::Duration::from_secs(2));

    client.set_activity(
        activity::ActivityBuilder::default()
            .state("part 2 (test)")
            .details("a placeholder")
            .assets(
                activity::AssetsBuilder::default()
                    .large_image("small-image")
                    .large_text("a thing")
                    .build(),
            )
            .build(),
    )?;
    std::thread::sleep(std::time::Duration::from_secs(2));

    client.disconnect()?;
    Ok(())
}
