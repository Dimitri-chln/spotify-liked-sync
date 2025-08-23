use std::ops::Deref;

use spotify_rs::model::{PlayableItem, playlist, track};

pub struct SavedTrack(pub track::SavedTrack);

pub struct PlaylistItem(pub playlist::PlaylistItem);

impl SavedTrack {
    pub fn uri(&self) -> &str {
        &self.track.uri
    }

    pub fn name(&self) -> &str {
        &self.track.name
    }
}

impl PlaylistItem {
    pub fn uri(&self) -> &str {
        match self.track {
            PlayableItem::Track(ref track) => &track.uri,
            PlayableItem::Episode(ref episode) => &episode.uri,
        }
    }

    pub fn name(&self) -> &str {
        match self.track {
            PlayableItem::Track(ref track) => &track.name,
            PlayableItem::Episode(ref episode) => &episode.name,
        }
    }
}

impl Deref for SavedTrack {
    type Target = track::SavedTrack;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for PlaylistItem {
    type Target = playlist::PlaylistItem;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq<PlaylistItem> for SavedTrack {
    fn eq(&self, other: &PlaylistItem) -> bool {
        self.uri() == other.uri()
    }
}
