use spotify_rs::Token;
use spotify_rs::client::Client;

use crate::compare::Compare;
use crate::config::Config;
use crate::env::Env;
use crate::progress::Progress;
use crate::track::{PlaylistItem, SavedTrack};
use crate::{Result, Spotify, TOKEN_FILE, utils};

pub async fn sync() -> Result<()> {
    let env = Env::load()?;
    let config = Config::load().await?;

    // Load the Spotify API token
    let token = self::load_token().progress("Loading credentials").await?;
    let refresh_token = token.refresh_secret().unwrap().to_owned();

    // Create a Spotify API client
    let spotify = Client::from_refresh_token(
        env.spotify_client_id(),
        Some(env.spotify_client_secret()),
        None,
        true,
        refresh_token,
    )
    .progress("Creating Spotify API client")
    .await?;

    // Load the user's saved tracks
    let saved_tracks = self::load_saved_tracks(&spotify)
        .progress("Loading saved tracks")
        .await?;

    // Load the sync playlist's tracks
    let sync_tracks = self::load_sync_tracks(config.sync_playlist_id(), &spotify)
        .progress("Loading sync tracks")
        .await?;

    // Compare both results
    let compare_result = Compare::new(saved_tracks.iter().rev(), sync_tracks.iter().rev());

    // Remove tracks from the sync playlist if necessary
    if !compare_result.to_remove().is_empty() {
        self::remove_tracks(
            config.sync_playlist_id(),
            compare_result.to_remove(),
            &spotify,
        )
        .progress(format!(
            "Removing {} tracks from the sync playlist",
            compare_result.to_remove().len()
        ))
        .await?;
    }

    // Add tracks to the sync playlist if necessary
    if !compare_result.to_add().is_empty() {
        self::add_tracks(config.sync_playlist_id(), compare_result.to_add(), &spotify)
            .progress(format!(
                "Adding {} tracks to the sync playlist",
                compare_result.to_add().len()
            ))
            .await?;
    }

    Ok(())
}

async fn load_token() -> Result<Token> {
    utils::load_json(TOKEN_FILE).await
}

async fn load_saved_tracks(spotify: &Spotify) -> Result<Vec<SavedTrack>> {
    let saved_tracks = spotify_rs::saved_tracks().get(spotify).await?;
    let saved_tracks = saved_tracks.get_all(spotify).await?;
    let saved_tracks = saved_tracks
        .into_iter()
        .flatten()
        .map(SavedTrack)
        .collect::<Vec<_>>();

    Ok(saved_tracks)
}

async fn load_sync_tracks(playlist_id: &str, spotify: &Spotify) -> Result<Vec<PlaylistItem>> {
    let sync_tracks = spotify_rs::playlist_items(playlist_id).get(spotify).await?;
    let sync_tracks = sync_tracks.get_all(spotify).await?;
    let sync_tracks = sync_tracks
        .into_iter()
        .flatten()
        .map(PlaylistItem)
        .collect::<Vec<_>>();

    Ok(sync_tracks)
}

async fn remove_tracks(
    playlist_id: &str,
    tracks: &[&PlaylistItem],
    spotify: &Spotify,
) -> Result<()> {
    for items in tracks.chunks(100) {
        let item_uris: Vec<_> = items.iter().rev().map(|track| track.uri()).collect();
        let item_names: Vec<_> = items.iter().rev().map(|track| track.name()).collect();

        println!(" - Removing: {}", item_names.join("\n - Removing: "));

        spotify_rs::remove_playlist_items(playlist_id, &item_uris)
            .send(spotify)
            .await?;
    }

    Ok(())
}

async fn add_tracks(playlist_id: &str, tracks: &[&SavedTrack], spotify: &Spotify) -> Result<()> {
    for items in tracks.chunks(100) {
        let item_uris: Vec<_> = items.iter().rev().map(|track| track.uri()).collect();
        let item_names: Vec<_> = items.iter().rev().map(|track| track.name()).collect();

        println!(" - Adding: {}", item_names.join("\n - Adding: "));

        spotify_rs::add_items_to_playlist(playlist_id, &item_uris)
            .position(0)
            .send(spotify)
            .await?;
    }

    Ok(())
}
