use super::error::Error;
use super::result::Result;
use super::{MediaProps, OsMediaProps};
use mpris::{DBusError, Metadata, PlayerFinder};

impl From<Metadata> for MediaProps {
    fn from(info: Metadata) -> Self {
        let mut res = Self::new();

        if let Some(artists) = info.artists() {
            res.artist = artists.join(", ");
        }

        if let Some(album_artists) = info.album_artists() {
            res.album_artist = album_artists.join(", ");
        }

        if let Some(title) = info.title() {
            res.title = title.to_string();
        }

        if let Some(album_title) = info.album_name() {
            res.album_title = album_title.to_string();
        }

        if let Some(subtitle) = info.url() {
            res.subtitle = subtitle.to_string();
        }

        if let Some(track) = info.track_number() {
            res.track_number = track;
        }

        res
    }
}

impl From<DBusError> for Error {
    fn from(err: DBusError) -> Self {
        Self::new(err.to_string())
    }
}

pub struct NixMediaManager {
    player_finder: PlayerFinder,
}

impl NixMediaManager {
    pub fn new() -> Result<Self> {
        match PlayerFinder::new() {
            Ok(player_finder) => Ok(Self { player_finder }),
            Err(err) => Err(err.into()),
        }
    }
}

impl OsMediaProps for NixMediaManager {
    fn currently_playing(&self) -> Result<MediaProps> {
        match self.player_finder.find_active() {
            Ok(player) => match player.get_metadata() {
                Ok(metadata) => Ok(MediaProps::from(metadata)),
                Err(err) => Err(err.into()),
            },
            Err(err) => Err(Error::new(err.to_string())),
        }
    }
}

// TODO: Async implementation?
