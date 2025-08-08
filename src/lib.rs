mod authorize;
pub mod cli;
mod compare;
mod config;
mod env;
mod error;
mod progress;
mod sync;
mod track;
mod utils;

pub use crate::error::Error;
pub use authorize::authorize;
pub use sync::sync;

use spotify_rs::{Token, UnknownFlow, client::Client};

const CONFIG_FILE: &str = "config.json";
const TOKEN_FILE: &str = "credentials.json";
const SCOPES: [&str; 4] = [
    "user-library-read",
    "playlist-modify-public",
    "playlist-read-private",
    "playlist-modify-private",
];

type Spotify = Client<Token, UnknownFlow>;
pub type Result<T> = std::result::Result<T, Error>;
