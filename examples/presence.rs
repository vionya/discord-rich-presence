use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DiscordIpcClient::new(&std::env::args().collect::<Vec<String>>()[1]);

    client.connect()?;
    client.set_activity(activity::Activity::new().state("foo").details("bar"))?;

    println!("Activity set! Press enter to exit...");

    let mut dummy = String::new();
    match std::io::stdin().read_line(&mut dummy) {
        _ => (),
    }

    client.close()?;
    Ok(())
}
