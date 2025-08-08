use std::env::VarError;

pub struct Env {
    /// Spotify client ID
    spotify_client_id: String,
    /// Spotify client secret
    spotify_client_secret: String,
    /// Redirect URL in the app's settings (on the Spotify API dashboard)
    spotify_redirect_uri: String,
}

impl Env {
    pub fn load() -> Result<Self, VarError> {
        Ok(Self {
            spotify_client_id: std::env::var("SPOTIFY_CLIENT_ID")?,
            spotify_client_secret: std::env::var("SPOTIFY_CLIENT_SECRET")?,
            spotify_redirect_uri: std::env::var("SPOTIFY_REDIRECT_URI")?,
        })
    }

    pub fn spotify_client_id(&self) -> &str {
        &self.spotify_client_id
    }

    pub fn spotify_client_secret(&self) -> &str {
        &self.spotify_client_secret
    }

    pub fn spotify_redirect_uri(&self) -> &str {
        &self.spotify_redirect_uri
    }
}
