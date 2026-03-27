//! Watermark component - Security overlay with text/logo.

use web_sys::Element;

/// Props for the Watermark component.
#[derive(Clone)]
pub struct WatermarkProps {
    pub text: Option<String>,
    pub image: Option<String>,
    pub opacity: f32,
    pub rotation: f32,
    pub gap: (u32, u32),
    pub offset: (u32, u32),
    pub font_size: u32,
    pub font_color: String,
    pub class: Option<String>,
}

impl Default for WatermarkProps {
    fn default() -> Self {
        Self {
            text: Some("CONFIDENTIAL".to_string()),
            image: None,
            opacity: 0.1,
            rotation: -30.0,
            gap: (100, 100),
            offset: (50, 50),
            font_size: 16,
            font_color: "#000000".to_string(),
            class: None,
        }
    }
}

/// Watermark component.
pub struct Watermark;

impl Watermark {
    /// Create a watermark overlay element.
    pub fn create(_props: WatermarkProps) -> Element {
        // TODO: Implement watermark with canvas or SVG pattern
        todo!("Watermark component implementation pending")
    }
}
