use serde::Deserialize;

use crate::{CONFIG_FILE, Result, utils};

#[derive(Deserialize)]
pub struct Config {
    sync_playlist_id: String,
}

impl Config {
    pub async fn load() -> Result<Self> {
        utils::load_json(CONFIG_FILE).await
    }

    pub fn sync_playlist_id(&self) -> &str {
        &self.sync_playlist_id
    }
}
