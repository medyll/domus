//! Media components for Domius.
//!
//! Video and audio player components.

pub mod audio_player;
pub mod video_player;

pub use audio_player::{audio_player, AudioPlayerProps};
pub use video_player::{video_player, VideoPlayerProps};
