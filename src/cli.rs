pub use clap::Parser;

#[derive(Parser, Debug)]
pub enum Command {
    Authorize,
    Sync,
}
