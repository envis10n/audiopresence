use serde::{Deserialize, Serialize};
use std::{future::Future, pin::Pin};

#[cfg(target_os = "windows")]
pub mod win;

#[cfg(target_os = "unix")]
pub mod unix;

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

pub trait OsMediaProps<E> {
    fn currently_playing(&self) -> Result<MediaProps, E>
    where
        E: std::error::Error;
}

pub trait AsyncOsMediaProps<E> {
    fn currently_playing(&self) -> Pin<Box<dyn Future<Output = Result<MediaProps, E>>>>;
}
