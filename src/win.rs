use super::error::Error;
use super::result::Result;
use super::{AsyncOsMediaProps, MediaProps, OsMediaProps, PlayerStatus, TimelineProps};
use std::pin::Pin;
use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSession as Session,
    GlobalSystemMediaTransportControlsSessionManager as SessionManager,
    GlobalSystemMediaTransportControlsSessionMediaProperties as MediaProperties,
    GlobalSystemMediaTransportControlsSessionPlaybackStatus as PlaybackStatus,
    GlobalSystemMediaTransportControlsSessionTimelineProperties as TimelineProperties,
};

impl From<TimelineProperties> for TimelineProps {
    fn from(info: TimelineProperties) -> Self {
        let mut res = Self::default();
        if let Ok(min) = info.MinSeekTime() {
            res.min_seek = min.Duration;
        }
        if let Ok(max) = info.MaxSeekTime() {
            res.max_seek = max.Duration;
        }
        if let Ok(pos) = info.Position() {
            res.position = pos.Duration;
        }
        if let Ok(start) = info.StartTime() {
            res.started = start.Duration;
        }
        if let Ok(end) = info.EndTime() {
            res.ended = end.Duration;
        }
        if let Ok(last_update) = info.LastUpdatedTime() {
            res.last_update = last_update.UniversalTime;
        }
        res
    }
}

impl From<MediaProperties> for MediaProps {
    fn from(info: MediaProperties) -> Self {
        let mut res = Self::new();
        res.artist = if let Ok(artist) = info.Artist() {
            artist.to_string()
        } else {
            String::new()
        };
        res.album_artist = if let Ok(album_artist) = info.AlbumArtist() {
            album_artist.to_string()
        } else {
            String::new()
        };
        res.album_title = if let Ok(album_title) = info.AlbumTitle() {
            album_title.to_string()
        } else {
            String::new()
        };
        res.title = if let Ok(title) = info.Title() {
            title.to_string()
        } else {
            String::new()
        };
        res.subtitle = if let Ok(subtitle) = info.Subtitle() {
            subtitle.to_string()
        } else {
            String::new()
        };
        res.album_track_count = if let Ok(album_track_count) = info.AlbumTrackCount() {
            album_track_count
        } else {
            -1
        };
        res.track_number = if let Ok(track_number) = info.TrackNumber() {
            track_number
        } else {
            -1
        };
        res
    }
}

pub struct MediaManager;

impl MediaManager {
    pub async fn get_media_properties_async() -> Result<MediaProps> {
        let session = MediaManager::get_session_async().await?;
        session
            .TryGetMediaPropertiesAsync()
            .map_err(|e| Error::new(e.to_string()))?
            .await
            .map(|res| MediaProps::from(res))
            .map_err(|e| Error::new(e.to_string()))
    }
    pub fn get_media_properties() -> Result<MediaProps> {
        let session = MediaManager::get_session()?;
        session
            .TryGetMediaPropertiesAsync()
            .map_err(|e| Error::new(e.to_string()))?
            .get()
            .map(|res| MediaProps::from(res))
            .map_err(|e| Error::new(e.to_string()))
    }
    pub async fn get_session_async() -> Result<Session> {
        let session_manager = SessionManager::RequestAsync()
            .map_err(|e| Error::new(e.to_string()))?
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        session_manager
            .GetCurrentSession()
            .map_err(|e| Error::new(e.to_string()))
    }
    pub fn get_session() -> Result<Session> {
        let session_manager = SessionManager::RequestAsync()
            .map_err(|e| Error::new(e.to_string()))?
            .get()
            .map_err(|e| Error::new(e.to_string()))?;
        session_manager
            .GetCurrentSession()
            .map_err(|e| Error::new(e.to_string()))
    }
    pub async fn get_player_status_async() -> Result<PlayerStatus> {
        let session = MediaManager::get_session_async().await?;
        let playback = session
            .GetPlaybackInfo()
            .map_err(|e| Error::new(e.to_string()))?;
        let status = playback
            .PlaybackStatus()
            .map_err(|e| Error::new(e.to_string()))?;
        match status {
            PlaybackStatus::Closed
            | PlaybackStatus::Stopped
            | PlaybackStatus::Changing
            | PlaybackStatus::Opened => Ok(PlayerStatus::None),
            PlaybackStatus::Playing | PlaybackStatus::Paused => {
                let tline: Option<TimelineProps> = {
                    if let Ok(timeline) = session.GetTimelineProperties() {
                        Some(TimelineProps::from(timeline))
                    } else {
                        None
                    }
                };
                if status == PlaybackStatus::Paused {
                    Ok(PlayerStatus::Paused(tline))
                } else {
                    Ok(PlayerStatus::Playing(tline))
                }
            }
            _ => Err(Error::new("Unable to get playback status")),
        }
    }
    pub fn get_player_status() -> Result<PlayerStatus> {
        let session = MediaManager::get_session()?;
        let playback = session
            .GetPlaybackInfo()
            .map_err(|e| Error::new(e.to_string()))?;
        let status = playback
            .PlaybackStatus()
            .map_err(|e| Error::new(e.to_string()))?;
        match status {
            PlaybackStatus::Closed
            | PlaybackStatus::Stopped
            | PlaybackStatus::Changing
            | PlaybackStatus::Opened => Ok(PlayerStatus::None),
            PlaybackStatus::Playing | PlaybackStatus::Paused => {
                let tline: Option<TimelineProps> = {
                    if let Ok(timeline) = session.GetTimelineProperties() {
                        Some(TimelineProps::from(timeline))
                    } else {
                        None
                    }
                };
                if status == PlaybackStatus::Paused {
                    Ok(PlayerStatus::Paused(tline))
                } else {
                    Ok(PlayerStatus::Playing(tline))
                }
            }
            _ => Err(Error::new("Unable to get playback status")),
        }
    }
}

impl OsMediaProps for MediaManager {
    fn currently_playing() -> Result<MediaProps> {
        MediaManager::get_media_properties()
    }
}

impl AsyncOsMediaProps for MediaManager {
    fn currently_playing() -> Pin<Box<dyn std::future::Future<Output = Result<MediaProps>>>> {
        Box::pin(async { MediaManager::get_media_properties_async().await })
    }
}

#[cfg(test)]
mod test {
    use super::{MediaManager, OsMediaProps};
    #[test]
    fn test_metadata() {
        let metadata = MediaManager::currently_playing().expect("Error fetching metadata.");
        println!("[Metadata] {:?}", metadata);
    }
}
