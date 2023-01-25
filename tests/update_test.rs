use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use std::error::Error;

#[test]
fn test_updating() -> Result<(), Box<dyn Error>> {
    let mut client = DiscordIpcClient::new("771124766517755954");
    client.connect(None)?;

    client.set_activity(
        activity::Activity::new()
            .state("part 1 (test)")
            .details("a placeholder")
            .assets(
                activity::Assets::new()
                    .large_image("large-image")
                    .large_text("a thing"),
            ),
    )?;
    std::thread::sleep(std::time::Duration::from_secs(2));

    client.set_activity(
        activity::Activity::new()
            .state("part 2 (test)")
            .details("a placeholder")
            .assets(
                activity::Assets::new()
                    .large_image("small-image")
                    .large_text("a thing"),
            ),
    )?;
    std::thread::sleep(std::time::Duration::from_secs(2));

    client.close()?;
    Ok(())
}
