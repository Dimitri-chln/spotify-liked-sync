use std::ops::Deref;

use spotify_rs::model::{PlayableItem, playlist, track};

pub struct SavedTrack(pub track::SavedTrack);

pub struct PlaylistItem(pub playlist::PlaylistItem);

impl SavedTrack {
    pub fn id(&self) -> &str {
        &self.track.id
    }
}

impl PlaylistItem {
    pub fn id(&self) -> &str {
        match self.track {
            PlayableItem::Track(ref track) => &track.id,
            PlayableItem::Episode(ref episode) => &episode.id,
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
        self.id() == other.id()
    }
}
