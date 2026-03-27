//! Carousel component - Image/content slider.

use domius_core::signal::Signal;
use web_sys::Element;

/// A single carousel item.
#[derive(Clone)]
pub struct CarouselItem {
    pub src: Option<String>,
    pub content: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
}

/// Props for the Carousel component.
pub struct CarouselProps {
    pub items: Vec<CarouselItem>,
    pub autoplay: bool,
    pub interval: u64,
    pub show_arrows: bool,
    pub show_dots: bool,
    pub infinite: bool,
    pub on_change: Option<Box<dyn Fn(usize)>>,
    pub class: Option<String>,
}

impl Default for CarouselProps {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            autoplay: false,
            interval: 5000,
            show_arrows: true,
            show_dots: true,
            infinite: true,
            on_change: None,
            class: None,
        }
    }
}

/// Carousel component.
pub struct Carousel;

impl Carousel {
    /// Create a carousel element.
    pub fn create(_props: CarouselProps) -> (Element, Signal<usize>) {
        // TODO: Implement carousel
        todo!("Carousel component implementation pending")
    }
}
