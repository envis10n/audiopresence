use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub mod error;
pub mod result;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "windows")]
pub mod win;

#[cfg(target_os = "linux")]
pub use linux::MediaManager;
#[cfg(target_os = "windows")]
pub use win::MediaManager;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerStatus {
    None,
    Playing(Option<TimelineProps>),
    Paused(Option<TimelineProps>),
}

impl Default for PlayerStatus {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimelineProps {
    pub min_seek: i64,
    pub max_seek: i64,
    pub position: i64,
    pub started: i64,
    pub ended: i64,
    pub last_update: i64,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct MediaProps {
    pub artist: String,
    pub title: String,
    pub album_artist: String,
    pub album_title: String,
    pub subtitle: String,
    pub album_track_count: i32,
    pub track_number: i32,
}

impl Default for MediaProps {
    fn default() -> Self {
        Self::new()
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
}

pub trait OsMediaProps {
    fn currently_playing() -> result::Result<MediaProps>;
    fn player_status() -> result::Result<PlayerStatus>;
    fn timeline() -> result::Result<TimelineProps>;
}

#[async_trait]
pub trait AsyncOsMediaProps {
    async fn currently_playing() -> result::Result<MediaProps>;
    async fn player_status() -> result::Result<PlayerStatus>;
    async fn timeline() -> result::Result<TimelineProps>;
}
