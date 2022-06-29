use super::error::Error;
use super::result::Result;
use super::{MediaProps, OsMediaProps, PlayerStatus, TimelineProps};
use mpris::{DBusError, Metadata, PlaybackStatus, Player, PlayerFinder, Progress};

impl TimelineProps {
    pub fn from_progress(tracker: &Progress) -> Result<Self> {
        let mut res = Self::default();
        let len = tracker.length().unwrap_or(Default::default());
        let updated = tracker.created_at().elapsed().as_secs() as i64;
        res.position = tracker.initial_position().as_secs() as i64;
        res.last_update = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
            - updated;
        res.min_seek = 0;
        res.max_seek = len.as_secs() as i64;
        Ok(res)
    }
}

impl From<&Metadata> for MediaProps {
    fn from(info: &Metadata) -> Self {
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
    pub fn get_player<'a>() -> Result<Player<'a>> {
        let finder = PlayerFinder::new().map_err(|e| Error::from(e))?;
        finder.find_active().map_err(|e| Error::new(e.to_string()))
    }
    pub fn get_media_properties() -> Result<MediaProps> {
        let player = MediaManager::get_player()?;
        let mut tracker = player.track_progress(1000).map_err(|e| Error::from(e))?;
        let (progress, _) = tracker.tick();
        let metadata = progress.metadata();
        Ok(MediaProps::from(metadata))
    }
    pub fn get_timeline() -> Result<TimelineProps> {
        let player = MediaManager::get_player()?;
        let mut tracker = player.track_progress(1000).map_err(|e| Error::from(e))?;
        let (progress, _) = tracker.tick();
        Ok(TimelineProps::from_progress(progress)?)
    }
    pub fn get_status() -> Result<PlayerStatus> {
        let player = MediaManager::get_player()?;
        let mut tracker = player.track_progress(1000).map_err(|e| Error::from(e))?;
        let (progress, _) = tracker.tick();
        let timeline = TimelineProps::from_progress(progress)?;
        match progress.playback_status() {
            PlaybackStatus::Stopped => Ok(PlayerStatus::None),
            PlaybackStatus::Playing => Ok(PlayerStatus::Playing(Some(timeline))),
            PlaybackStatus::Paused => Ok(PlayerStatus::Paused(Some(timeline))),
        }
    }
}

impl OsMediaProps for MediaManager {
    fn currently_playing() -> Result<MediaProps> {
        MediaManager::get_media_properties()
    }
    fn player_status() -> Result<PlayerStatus> {
        MediaManager::get_status()
    }
    fn timeline() -> Result<TimelineProps> {
        MediaManager::get_timeline()
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
    #[test]
    fn test_status() {
        let status = MediaManager::get_status().expect("Error getting status.");
        println!("[Status] {:?}", status);
    }
}
