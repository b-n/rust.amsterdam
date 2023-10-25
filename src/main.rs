use clap::{command, Parser, Subcommand};
use inviteify::{ChannelInviteRequest, Inviteify};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, env = "DISCORD_BOT_TOKEN")]
    token: String,
    #[arg(long, env = "DISCORD_CLIENT_ID")]
    client_id: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Build {
        #[arg(long, env = "DISCORD_CHANNEL_ID")]
        channel_id: String,
    },
    BotInviteLink {
        #[arg(long, env = "DISCORD_SERVER_ID")]
        server_id: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match &args.command {
        Commands::Build { channel_id } => build(&args.token, &args.client_id, channel_id).await,
        Commands::BotInviteLink { server_id } => {
            bot_invite_link(&args.token, &args.client_id, server_id).await
        }
    }
}

async fn build(
    token: &str,
    client_id: &str,
    channel_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let _service = Inviteify::new(client_id, token)?;

    // Clean/create build folder
    // Recursive copy static to build
    // Generate QR invite link

    //let req = ChannelInviteRequest {
    //max_age: 86400,
    //..ChannelInviteRequest::default()
    //};

    //let invite_code = service
    //.check_and_generate_invite(&discord_channel_id, &req)
    //.await?;

    println!("Output generated in build/");

    Ok(())
}

async fn bot_invite_link(
    token: &str,
    client_id: &str,
    server_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let service = Inviteify::new(client_id, token)?;

    println!(
        "Click the following link and follow the prompts to authorize the bot:\n{}",
        service.authorization_link(server_id)
    );
    Ok(())
}
