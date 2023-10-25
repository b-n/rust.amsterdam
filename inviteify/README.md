# Inviteify

A rust library that can be used to manage discord invite links.

## Config

The authentication details can be generated via the [discord application portal](https://discord.com/developers/applications?new_application=true).

See [the discord docs](https://discord.com/developers/docs/getting-started) for more info.

## Examples

The below is contrived. You should only need to register the bot once, however
it is here to show the usaged of authorization link generation.

```rust
use inviteify::{ChannelInviteRequest, Inviteify};
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("DISCORD_BOT_TOKEN").unwrap();
    let client_id = std::env::var("DISCORD_CLIENT_ID").unwrap();
    let discord_channel_id = std::env::var("DISCORD_CHANNEL_ID").unwrap();
    let discord_server_id = std::env::var("DISCORD_SERVER_ID").unwrap();

    let mut service = Inviteify::new(&client_id, &token)?;

    println!(
        "Click this link to authorize the bot: {}",
        service.authorization_link(&discord_server_id)
    );
    println!("Press enter when ready to continue.");

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let req = ChannelInviteRequest {
        max_age: 86400,
        ..ChannelInviteRequest::default()
    };

    let invite_code = service
        .check_and_generate_invite(&discord_channel_id, &req)
        .await?;

    println!("Invite code: {}", invite_code);

    Ok(())
}
```
