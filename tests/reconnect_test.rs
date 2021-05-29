use discord_rich_presence::{new_client, DiscordIpc};
use serde_json::json;
use std::error::Error;

#[test]
fn test_reconnect() -> Result<(), Box<dyn Error>> {
    let mut client = new_client("771124766517755954")?;
    client.connect()?;

    loop {
        let payload = json!({
            "state": "part 1 (test)",
            "details": "a placeholder",
            "assets": {
                "large_text": "a thing",
                "large_image": "large-image"
            }
        });

        match client.set_activity(payload) {
            Ok(_) => (),
            Err(_) => match client.get_valid_path()? {
                Some(_) => {
                    client.reconnect()?;
                },
                None => (),
            },
        }

        std::thread::sleep(std::time::Duration::from_secs(2));
    }

    #[allow(unreachable_code)]
    Ok(())
}
