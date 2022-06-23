use std::sync::{Arc, Mutex};
use windows::Foundation::TypedEventHandler;
use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSession,
    GlobalSystemMediaTransportControlsSessionManager as SessionManager,
    MediaPropertiesChangedEventArgs,
    GlobalSystemMediaTransportControlsSessionMediaProperties as MediaProperties,
};

use tokio::time::{self, Duration};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct MediaProps {
    artist: String,
    title: String,
    album_artist: String,
    album_title: String,
    subtitle: String,
    album_track_count: i32,
    track_number: i32,
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

fn main() {
    let props = Arc::new(Mutex::new(MediaProps::new()));
    let task = async move {
        let session_manager = SessionManager::RequestAsync().unwrap().await.unwrap();
        let session = session_manager.GetCurrentSession().unwrap();
        let props = props.clone();
        if let Ok(info) = session.TryGetMediaPropertiesAsync().unwrap().await {
            let props = props.clone();
            let mut prop = props.lock().unwrap();
            (*prop).update_from(info);
            println!("[Now Playing] {:?}", *prop);
        }
        let _token = session
            .MediaPropertiesChanged(TypedEventHandler::new(
                move |sender: &Option<GlobalSystemMediaTransportControlsSession>,
                      _args: &Option<MediaPropertiesChangedEventArgs>| {
                    let props = props.clone();
                    if let Some(session) = sender {
                        match session.TryGetMediaPropertiesAsync().unwrap().get() {
                            Ok(info) => {
                                let mut mprops = MediaProps::new();
                                mprops.update_from(info.clone());
                                let mut props_lock = props.lock().unwrap();
                                let props_clone = (*props_lock).clone();
                                if props_clone != mprops {
                                    (*props_lock).update_from(info);
                                    println!("[Now Playing] {:?}", *props_lock);
                                }
                            }
                            Err(err) => {
                                println!("ERROR: {:?}", err);
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
    };
    let ctrl_c = async {
        tokio::signal::ctrl_c().await.unwrap();
    };
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            tokio::select! {
                _ = task => {},
                _ = ctrl_c => {
                    println!("Ctrl-C...");
                },
            }
        });
}
