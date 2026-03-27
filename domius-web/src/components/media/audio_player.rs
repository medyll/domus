//! Audio player component for Domius.
//!
//! Simple wrapper around HTML5 audio element.

use web_sys::Element;

/// Audio player props.
#[derive(Clone)]
pub struct AudioPlayerProps {
    /// Audio source URL
    pub src: String,
    /// Show browser controls (default: true)
    pub controls: bool,
    /// Auto-play on load
    pub auto_play: bool,
    /// Loop audio
    pub loop_audio: bool,
    /// CSS class
    pub class: Option<String>,
}

impl Default for AudioPlayerProps {
    fn default() -> Self {
        Self {
            src: String::new(),
            controls: true,
            auto_play: false,
            loop_audio: false,
            class: None,
        }
    }
}

/// Build an audio player component.
///
/// This is a simple wrapper around the HTML5 audio element.
/// For advanced controls, use the native `controls` attribute or build custom controls
/// with domius_core signals and effects.
///
/// # Example
///
/// ```ignore
/// use domius_web::components::audio_player::{audio_player, AudioPlayerProps};
///
/// let node = audio_player(AudioPlayerProps {
///     src: "https://example.com/audio.mp3".to_string(),
///     controls: true,
///     ..Default::default()
/// });
/// ```
pub fn audio_player(props: AudioPlayerProps) -> Element {
    let document = web_sys::window().unwrap().document().unwrap();
    
    // Create audio element
    let audio: Element = document.create_element("audio").unwrap();
    
    if let Some(class) = &props.class {
        audio.set_class_name(class);
    }
    if props.controls {
        audio.set_attribute("controls", "").ok();
    }
    if props.auto_play {
        audio.set_attribute("autoplay", "").ok();
    }
    if props.loop_audio {
        audio.set_attribute("loop", "").ok();
    }
    audio.set_attribute("src", &props.src).ok();

    audio
}
