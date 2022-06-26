use super::error::Error;
use super::result::Result;
use super::{AsyncOsMediaProps, MediaProps, OsMediaProps};
use std::pin::Pin;
use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSession,
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
    pub fn session(&self) -> GlobalSystemMediaTransportControlsSession {
        self.session_manager.GetCurrentSession().unwrap()
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

/*

use std::sync::Arc;
use tokio::time::{self, Duration};
use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSession,
    GlobalSystemMediaTransportControlsSessionManager as SessionManager,
    GlobalSystemMediaTransportControlsSessionMediaProperties as MediaProperties,
    MediaPropertiesChangedEventArgs,
};

pub async fn register_handler<'a, T>(handler: T)
where
    T: Fn(MediaProps) -> () + 'static + Send + Sync,
{
    let handler = Arc::new(Box::new(handler));
    let session_manager = SessionManager::RequestAsync().unwrap().await.unwrap();
    let session = session_manager.GetCurrentSession().unwrap();
    if let Ok(info) = session.TryGetMediaPropertiesAsync().unwrap().await {
        let val = MediaProps::from(info);
        handler(val);
    }
    let _token = session
        .MediaPropertiesChanged(TypedEventHandler::new(
            move |sender: &Option<GlobalSystemMediaTransportControlsSession>,
                  _args: &Option<MediaPropertiesChangedEventArgs>| {
                let handler = handler.clone();
                if let Some(session) = sender {
                    match session.TryGetMediaPropertiesAsync().unwrap().get() {
                        Ok(info) => {
                            let val = MediaProps::from(info);
                            handler(val);
                        }
                        Err(err) => {
                            println!("ERROR: {}", err);
                        }
                    }
                }
                Ok(())
            },
        ))
        .unwrap();
    loop {
        tokio::time::sleep(Duration::from_millis(1000)).await;
    }
}
*/

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
