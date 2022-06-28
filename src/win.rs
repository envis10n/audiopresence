use super::error::Error;
use super::result::Result;
use super::{AsyncOsMediaProps, MediaProps, OsMediaProps};
use std::pin::Pin;
use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSessionManager as SessionManager,
    GlobalSystemMediaTransportControlsSessionMediaProperties as MediaProperties,
};

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
        let session_manager = SessionManager::RequestAsync()
            .map_err(|e| Error::new(e.to_string()))?
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        let session = session_manager
            .GetCurrentSession()
            .map_err(|e| Error::new(e.to_string()))?;
        session
            .TryGetMediaPropertiesAsync()
            .map_err(|e| Error::new(e.to_string()))?
            .await
            .map(|res| MediaProps::from(res))
            .map_err(|e| Error::new(e.to_string()))
    }
    pub fn get_media_properties() -> Result<MediaProps> {
        let session_manager = SessionManager::RequestAsync()
            .map_err(|e| Error::new(e.to_string()))?
            .get()
            .map_err(|e| Error::new(e.to_string()))?;
        let session = session_manager
            .GetCurrentSession()
            .map_err(|e| Error::new(e.to_string()))?;
        session
            .TryGetMediaPropertiesAsync()
            .map_err(|e| Error::new(e.to_string()))?
            .get()
            .map(|res| MediaProps::from(res))
            .map_err(|e| Error::new(e.to_string()))
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
