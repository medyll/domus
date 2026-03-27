//! Video player component for Domius.
//!
//! Simple wrapper around HTML5 video element.

use web_sys::Element;

/// Video player props.
#[derive(Clone)]
pub struct VideoPlayerProps {
    /// Video source URL
    pub src: String,
    /// Show browser controls (default: true)
    pub controls: bool,
    /// Auto-play on load
    pub auto_play: bool,
    /// Loop video
    pub loop_video: bool,
    /// Poster image URL
    pub poster: Option<String>,
    /// CSS class
    pub class: Option<String>,
}

impl Default for VideoPlayerProps {
    fn default() -> Self {
        Self {
            src: String::new(),
            controls: true,
            auto_play: false,
            loop_video: false,
            poster: None,
            class: None,
        }
    }
}

/// Build a video player component.
///
/// This is a simple wrapper around the HTML5 video element.
/// For advanced controls, use the native `controls` attribute or build custom controls
/// with domius_core signals and effects.
///
/// # Example
///
/// ```ignore
/// use domius_web::components::video_player::{video_player, VideoPlayerProps};
///
/// let node = video_player(VideoPlayerProps {
///     src: "https://example.com/video.mp4".to_string(),
///     controls: true,
///     ..Default::default()
/// });
/// ```
pub fn video_player(props: VideoPlayerProps) -> Element {
    let document = web_sys::window().unwrap().document().unwrap();
    
    // Create video element
    let video: Element = document.create_element("video").unwrap();
    
    if let Some(class) = &props.class {
        video.set_class_name(class);
    }
    if props.controls {
        video.set_attribute("controls", "").ok();
    }
    if props.auto_play {
        video.set_attribute("autoplay", "").ok();
    }
    if props.loop_video {
        video.set_attribute("loop", "").ok();
    }
    video.set_attribute("src", &props.src).ok();
    
    if let Some(poster) = &props.poster {
        video.set_attribute("poster", poster).ok();
    }

    video
}
