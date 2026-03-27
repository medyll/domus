//! ColorPicker component - Color selection interface.

use domius_core::signal::{signal, Signal};
use web_sys::Element;

/// Color format.
#[derive(Clone, PartialEq)]
pub enum ColorFormat {
    Hex,
    Rgb,
    Hsl,
    Hsv,
}

/// Color representation.
#[derive(Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return None;
        }
        Some(Self {
            r: u8::from_str_radix(&hex[0..2], 16).ok()?,
            g: u8::from_str_radix(&hex[2..4], 16).ok()?,
            b: u8::from_str_radix(&hex[4..6], 16).ok()?,
            a: 255,
        })
    }
}

/// Props for the ColorPicker component.
pub struct ColorPickerProps {
    pub value: Signal<Color>,
    pub format: ColorFormat,
    pub show_alpha: bool,
    pub preset_colors: Vec<Color>,
    pub on_change: Option<Box<dyn Fn(Color)>>,
    pub class: Option<String>,
}

impl Default for ColorPickerProps {
    fn default() -> Self {
        Self {
            value: signal(Color::new(0, 0, 0, 255)),
            format: ColorFormat::Hex,
            show_alpha: false,
            preset_colors: vec![
                Color::new(255, 0, 0, 255),
                Color::new(0, 255, 0, 255),
                Color::new(0, 0, 255, 255),
                Color::new(255, 255, 0, 255),
                Color::new(0, 255, 255, 255),
                Color::new(255, 0, 255, 255),
            ],
            on_change: None,
            class: None,
        }
    }
}

/// ColorPicker component.
pub struct ColorPicker;

impl ColorPicker {
    /// Create a color picker element.
    pub fn create(_props: ColorPickerProps) -> (Element, Signal<Color>) {
        // TODO: Implement color picker with hue/saturation selectors
        todo!("ColorPicker component implementation pending")
    }
}
