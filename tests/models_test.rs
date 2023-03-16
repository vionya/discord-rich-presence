use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use std::error::Error;

#[test]
fn test_models() -> Result<(), Box<dyn Error>> {
    let mut client = DiscordIpcClient::new("771124766517755954");
    client.connect()?;

    let mut activity = activity::ActivityBuilder::default()
        .state("State")
        .details("Details")
        .assets(
            activity::AssetsBuilder::default()
                .large_image("large-image")
                .large_text("Large text")
                .build(),
        )
        .buttons(vec![activity::Button::new(
            "A button",
            "https://example.com",
        )])
        .timestamps(activity::models::Timestamps::new(Some(1), None))
        .secrets(
            activity::SecretsBuilder::default()
                .join_secret("abc")
                .build(),
        )
        .build();

    activity.set_party(
        activity::PartyBuilder::default()
            .id("some-id")
            .size([1, 3])
            .build(),
    );

    println!("{:#?}", client.set_activity(activity)?);

    std::thread::park();
    // std::thread::sleep(std::time::Duration::from_secs(10));

    client.close()?;
    Ok(())
}
