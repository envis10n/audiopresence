use super::error::Error;
use super::result::Result;
use super::{AsyncOsMediaProps, MediaProps, OsMediaProps};
use std::pin::Pin;
use windows::Foundation::{EventRegistrationToken, TypedEventHandler};
use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSession as Session,
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

pub struct MediaManager {
    session_manager: SessionManager,
}

impl MediaManager {
    pub fn new() -> Self {
        Self {
            session_manager: SessionManager::RequestAsync().unwrap().get().unwrap(),
        }
    }
    pub fn session(&self) -> Session {
        self.session_manager.GetCurrentSession().unwrap()
    }
    pub fn on_media_change<F>(&self, f: F) -> Result<EventRegistrationToken>
    where
        F: Fn(MediaProps) -> () + Send + Sync + 'static,
    {
        let session = self.session();
        session
            .MediaPropertiesChanged(TypedEventHandler::new(
                move |_sender: &Option<Session>, _args| {
                    if let Some(session) = _sender {
                        let res = match session.TryGetMediaPropertiesAsync().unwrap().get() {
                            Ok(info) => {
                                f(MediaProps::from(info));
                                Ok(())
                            }
                            Err(e) => Err(e),
                        };
                        res
                    } else {
                        Ok(())
                    }
                },
            ))
            .map_err(|err| Error::new(err.to_string()))
    }
    pub fn cancel_media_change(&self, token: EventRegistrationToken) -> Result<()> {
        let session = self.session();
        session
            .RemoveMediaPropertiesChanged(token)
            .map_err(|e| Error::new(e.to_string()))
    }
}

impl OsMediaProps for MediaManager {
    fn currently_playing(&self) -> Result<MediaProps> {
        let session = self.session();
        match session.TryGetMediaPropertiesAsync().unwrap().get() {
            Ok(info) => Ok(MediaProps::from(info)),
            Err(e) => Err(Error::new(e.to_string())),
        }
    }
}

impl AsyncOsMediaProps for MediaManager {
    fn currently_playing(&self) -> Pin<Box<dyn std::future::Future<Output = Result<MediaProps>>>> {
        let session = self.session();
        Box::pin(async move {
            match session.TryGetMediaPropertiesAsync().unwrap().await {
                Ok(info) => Ok(MediaProps::from(info)),
                Err(e) => Err(Error::new(e.to_string())),
            }
        })
    }
}

#[cfg(test)]
mod test {
    use super::{MediaManager, OsMediaProps};
    #[test]
    fn test_metadata() {
        let manager = MediaManager::new();
        let metadata = manager
            .currently_playing()
            .expect("Error fetching metadata.");
        println!("[Metadata] {:?}", metadata);
    }
}
