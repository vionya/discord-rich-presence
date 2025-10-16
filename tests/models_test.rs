use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use std::error::Error;

#[test]
fn test_models() -> Result<(), Box<dyn Error>> {
    let mut client = DiscordIpcClient::new("771124766517755954");
    client.connect()?;

    let activity = activity::Activity::new()
        .state("A test")
        .details("A placeholder")
        .assets(
            activity::Assets::new()
                .large_image("large-image")
                .large_text("Large text")
                .large_url("https://example.com")
                .small_image("https://picsum.photos/id/128/200")
                .small_text("Small image")
                .small_url("https://picsum.photos/id/128/200")
        )
        .buttons(vec![activity::Button::new(
            "A button",
            "https://github.com",
        )]);
    client.set_activity(activity)?;

    std::thread::sleep(std::time::Duration::from_secs(10));

    client.close()?;
    Ok(())
}
