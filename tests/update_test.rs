use discord_rich_presence::{new_client, DiscordIpc};
use serde_json::json;
use std::error::Error;

#[test]
fn test_updating() -> Result<(), Box<dyn Error>> {
    let mut client = new_client("771124766517755954".to_string())?;

    client.set_activity(json!({
        "state": "part 1 (test)",
        "details": "a placeholder",
        "assets": {
            "large_text": "a thing",
            "large_image": "large-image"
        }
    }))?;
    std::thread::sleep(std::time::Duration::from_secs(2));

    client.set_activity(json!({
        "state": "part 2 (test)",
        "details": "a placeholder",
        "assets": {
            "large_text": "a thing",
            "large_image": "small-image"
        }
    }))?;
    std::thread::sleep(std::time::Duration::from_secs(2));

    client.close()?;
    Ok(())
}
