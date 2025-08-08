use clap::Parser;
use spotify_liked_sync::cli::Command;
use spotify_liked_sync::{Result, authorize, sync};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let command = Command::parse();

    match command {
        Command::Authorize => authorize().await,
        Command::Sync => sync().await,
    }
}
