use discord_rich_presence::{activity, DiscordIpcClient};
use std::error::Error;

#[test]
fn test_models() -> Result<(), Box<dyn Error>> {
    let mut client = DiscordIpcClient::new("771124766517755954");

    let user_data = &client.connect()?["data"]["user"];
    let (hash, id) = (
        &user_data["avatar"].as_str().unwrap(),
        &user_data["id"].as_str().unwrap(),
    );
    let avatar_url = format!(
        "https://cdn.discordapp.com/avatars/{}/{}.png?size=1024",
        id, hash
    );

    let mut activity = activity::ActivityBuilder::default()
        .state("State")
        .details("Details")
        .assets(
            activity::AssetsBuilder::default()
                // try with a static asset
                .large_image("large-image")
                .large_text("Large text")
                // try with a dynamic URL
                .small_image(&avatar_url)
                .small_text("small text")
                .build(),
        )
        .buttons(vec![activity::Button::new("go to avatar URL", &avatar_url)])
        .timestamps(activity::Timestamps::new(Some(1), None))
        // .secrets(
        //     activity::SecretsBuilder::default()
        //         .join_secret("abc")
        //         .build(),
        // )
        .build();

    activity.set_party(
        activity::PartyBuilder::default()
            .id("some-id")
            .size([1, 3])
            .build(),
    );

    client.set_activity(activity)?;
    dbg!(client.connected());
    // client.disconnect()?;
    dbg!(client.connected());
    std::thread::park();

    Ok(())
}
