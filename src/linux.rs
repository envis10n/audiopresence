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

pub struct MediaManager;

impl MediaManager {
    pub fn get_media_properties() -> Result<MediaProps> {
        let finder = PlayerFinder::new().map_err(|e| Error::from(e))?;
        let player = finder
            .find_active()
            .map_err(|e| Error::new(e.to_string()))?;
        player
            .get_metadata()
            .map(|res| MediaProps::from(res))
            .map_err(|e| e.into())
    }
}

impl OsMediaProps for MediaManager {
    fn currently_playing() -> Result<MediaProps> {
        MediaManager::get_media_properties()
    }
}

// TODO: Async implementation?

#[cfg(test)]
mod test {
    use super::{MediaManager, OsMediaProps};
    #[test]
    fn test_metadata() {
        let metadata = MediaManager::currently_playing().expect("Error getting metadata.");
        println!("[Metadata] {:?}", metadata);
    }
}
