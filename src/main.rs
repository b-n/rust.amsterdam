use clap::{command, Parser, Subcommand};
use inviteify::{ChannelInviteRequest, Inviteify};
use std::path::{Path, PathBuf};
use std::{env, fs, io};

fn default_build_path() -> PathBuf {
    let mut p = env::current_dir().expect("Unable to locate current working directory");
    p.push("build");
    p
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

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
        #[arg(long, default_value = default_build_path().into_os_string())]
        output_dir: PathBuf,
    },
    BotInviteLink {
        #[arg(long, env = "DISCORD_SERVER_ID")]
        server_id: String,
    },
}

#[derive(Debug)]
struct BuildError(String);

impl std::error::Error for BuildError {}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BuildError: {}", self.0)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match &args.command {
        Commands::Build {
            channel_id,
            output_dir,
        } => build(&args.token, &args.client_id, channel_id, output_dir).await,
        Commands::BotInviteLink { server_id } => {
            bot_invite_link(&args.token, &args.client_id, server_id).await
        }
    }
}

async fn build(
    token: &str,
    client_id: &str,
    channel_id: &str,
    output_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut service = Inviteify::new(client_id, token)?;

    // Clean build dir and create anew
    if output_dir.exists() {
        println!("Cleaning output directory");
        fs::remove_dir_all(output_dir)
            .map_err(|_| BuildError("Failed to clean build directory".to_string()))?;
    }
    fs::create_dir(output_dir)?;

    // Generate QR Code
    let req = ChannelInviteRequest {
        max_age: 86400,
        ..ChannelInviteRequest::default()
    };

    let invite = service.check_and_generate_invite(channel_id, &req).await?;

    let qrcode = service
        .invite_qrcode(&invite)?
        .render::<qrcode::render::svg::Color>()
        .min_dimensions(200, 200)
        .build();
    fs::write(output_dir.join("discord_invite.svg"), qrcode)?;

    // Recursive copy static/ to build output
    let static_path = env::current_dir().unwrap().join("static");
    if static_path.is_dir() {
        copy_dir_all(&static_path, output_dir)?
    }

    println!("Build generated in {}", output_dir.display());
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
