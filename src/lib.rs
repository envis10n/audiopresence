use std::sync::Arc;
use windows::Foundation::TypedEventHandler;
use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSession,
    GlobalSystemMediaTransportControlsSessionManager as SessionManager,
    GlobalSystemMediaTransportControlsSessionMediaProperties as MediaProperties,
    MediaPropertiesChangedEventArgs,
};

use tokio::time::{self, Duration};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct MediaProps {
    artist: String,
    title: String,
    album_artist: String,
    album_title: String,
    subtitle: String,
    album_track_count: i32,
    track_number: i32,
}

impl From<MediaProperties> for MediaProps {
    fn from(a: MediaProperties) -> Self {
        let mut res = Self::new();
        res.update_from(a.clone());
        res
    }
}

impl MediaProps {
    pub fn new() -> Self {
        Self {
            artist: String::new(),
            title: String::new(),
            album_artist: String::new(),
            album_title: String::new(),
            subtitle: String::new(),
            album_track_count: -1,
            track_number: -1,
        }
    }
    pub fn update_from(&mut self, info: MediaProperties) {
        self.artist = if let Ok(artist) = info.Artist() {
            artist.to_string()
        } else {
            String::new()
        };
        self.album_artist = if let Ok(album_artist) = info.AlbumArtist() {
            album_artist.to_string()
        } else {
            String::new()
        };
        self.album_title = if let Ok(album_title) = info.AlbumTitle() {
            album_title.to_string()
        } else {
            String::new()
        };
        self.title = if let Ok(title) = info.Title() {
            title.to_string()
        } else {
            String::new()
        };
        self.subtitle = if let Ok(subtitle) = info.Subtitle() {
            subtitle.to_string()
        } else {
            String::new()
        };
        self.album_track_count = if let Ok(album_track_count) = info.AlbumTrackCount() {
            album_track_count
        } else {
            -1
        };
        self.track_number = if let Ok(track_number) = info.TrackNumber() {
            track_number
        } else {
            -1
        };
    }
}

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
        time::sleep(Duration::from_millis(1000)).await;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[tokio::test]
    async fn test_handler() {
        tokio::select! {
            _ = register_handler(|props| {
                println!("[Now Playing] {:?}", props);
            }) => {}
            _ = tokio::signal::ctrl_c() => {
                println!("Ctrl-c...");
            }
        }
    }
}
