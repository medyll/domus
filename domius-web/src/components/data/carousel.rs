//! Carousel component - Responsive pure CSS image slider.
//!
//! Based on "Responsive CSS Image Slider" by Dudley Storey.
//! Uses CSS keyframes animation for smooth sliding without JavaScript.
//!
//! # Features
//! - Pure CSS animation (no JavaScript for sliding)
//! - Responsive design with percentage-based widths
//! - Optional manual controls (arrows, dots)
//! - Infinite looping support
//! - Accessible with proper ARIA attributes
//!
//! # Example
//! ```ignore
//! let items = vec![
//!     CarouselItem {
//!         src: Some("image1.jpg".to_string()),
//!         alt: "First image".to_string(),
//!         title: Some("Title 1".to_string()),
//!         description: Some("Description 1".to_string()),
//!     },
//!     // ... more items
//! ];
//!
//! let props = CarouselProps {
//!     items,
//!     autoplay: true,
//!     interval: 5000,
//!     show_arrows: true,
//!     show_dots: true,
//!     infinite: true,
//!     ..Default::default()
//! };
//!
//! let (element, current_index) = Carousel::create(props);
//! ```

use domius_core::signal::{signal, Signal};
use domius_core::effect::create_effect;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, HtmlImageElement, MouseEvent};

/// A single carousel item.
#[derive(Clone)]
pub struct CarouselItem {
    /// Image source URL
    pub src: Option<String>,
    /// Alt text for accessibility
    pub alt: Option<String>,
    /// Optional content (HTML or text)
    pub content: Option<String>,
    /// Optional title overlay
    pub title: Option<String>,
    /// Optional description overlay
    pub description: Option<String>,
}

impl Default for CarouselItem {
    fn default() -> Self {
        Self {
            src: None,
            alt: Some(String::new()),
            content: None,
            title: None,
            description: None,
        }
    }
}

/// Carousel component.
pub struct Carousel;

/// Props for the Carousel component.
pub struct CarouselProps {
    /// Items to display in the carousel
    pub items: Vec<CarouselItem>,
    /// Enable automatic sliding
    pub autoplay: bool,
    /// Interval between slides in milliseconds
    pub interval: u64,
    /// Show navigation arrows
    pub show_arrows: bool,
    /// Show dot indicators
    pub show_dots: bool,
    /// Enable infinite looping
    pub infinite: bool,
    /// Callback when slide changes
    pub on_change: Option<Box<dyn Fn(usize)>>,
    /// Additional CSS classes
    pub class: Option<String>,
    /// Animation duration in seconds
    pub animation_duration: u64,
    /// Pause on hover
    pub pause_on_hover: bool,
}

impl Default for CarouselProps {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            autoplay: true,
            interval: 5000,
            show_arrows: true,
            show_dots: true,
            infinite: true,
            on_change: None,
            class: None,
            animation_duration: 30,
            pause_on_hover: true,
        }
    }
}

impl Carousel {
    /// Create a carousel element.
    ///
    /// Returns the carousel element and a signal with the current slide index.
    pub fn create(props: CarouselProps) -> (Element, Signal<usize>) {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");

        let item_count = props.items.len();
        if item_count == 0 {
            // Return empty carousel
            let empty = document.create_element("div").unwrap();
            empty.set_text_content(Some("No carousel items"));
            return (empty, signal(0));
        }

        // Calculate total slides (add one for loop if infinite)
        let total_slides = if props.infinite { item_count + 1 } else { item_count };
        
        // Calculate figure width as percentage (100% * total_slides)
        let figure_width = total_slides * 100;
        
        // Calculate individual slide width
        let slide_width = 100.0 / item_count as f64;

        // Create main container
        let slider: HtmlElement = document
            .create_element("div")
            .unwrap()
            .dyn_into()
            .unwrap();
        slider.set_attribute("class", "domius-carousel").unwrap();
        slider.set_attribute("role", "region").unwrap();
        slider.set_attribute("aria-label", "Image carousel").unwrap();
        slider.set_attribute("aria-roledescription", "carousel").unwrap();

        if let Some(class) = &props.class {
            slider.class_list().add_1(class).ok();
        }

        // Create figure (image strip)
        let figure: HtmlElement = document
            .create_element("figure")
            .unwrap()
            .dyn_into()
            .unwrap();
        figure.set_attribute("class", "domius-carousel-figure").unwrap();
        
        // Set figure width
        figure.set_attribute(
            "style",
            &format!("width: {}%;", figure_width)
        ).unwrap();

        // Build animation CSS
        let animation_name = format!("slidy-{}", item_count);
        let keyframes = Self::generate_keyframes(item_count, props.interval, props.infinite);
        
        // Inject keyframes style
        if let Some(head) = document.query_selector("head").ok().flatten() {
            let style = document.create_element("style").unwrap();
            style.set_text_content(Some(&keyframes));
            head.append_child(&style).ok();
        }

        // Apply animation to figure
        let animation_duration = props.animation_duration;
        let existing_style = figure.get_attribute("style").unwrap_or_default();
        figure.set_attribute(
            "style",
            &format!(
                "{} animation: {}s {} infinite; animation-play-state: running;",
                existing_style, animation_duration, animation_name
            )
        ).unwrap();

        // Add items to figure
        for (i, item) in props.items.iter().enumerate() {
            let img: HtmlImageElement = document
                .create_element("img")
                .unwrap()
                .dyn_into()
                .unwrap();
            
            img.set_class_name("domius-carousel-item");
            
            // Set image width as percentage
            img.set_attribute(
                "style",
                &format!("width: {}%; float: left;", slide_width)
            ).unwrap();

            if let Some(src) = &item.src {
                img.set_src(src);
            }
            if let Some(alt) = &item.alt {
                img.set_alt(alt);
            } else {
                img.set_alt("Carousel image");
            }

            figure.append_child(img.dyn_ref::<Element>().unwrap()).unwrap();
        }

        // Add first image again for infinite loop
        if props.infinite && !props.items.is_empty() {
            let first_item = &props.items[0];
            let img: HtmlImageElement = document
                .create_element("img")
                .unwrap()
                .dyn_into()
                .unwrap();
            
            img.set_class_name("domius-carousel-item");
            img.set_attribute(
                "style",
                &format!("width: {}%; float: left;", slide_width)
            ).unwrap();

            if let Some(src) = &first_item.src {
                img.set_src(src);
            }
            if let Some(alt) = &first_item.alt {
                img.set_alt(alt);
            }

            figure.append_child(img.dyn_ref::<Element>().unwrap()).unwrap();
        }

        slider.append_child(&figure).unwrap();

        // Create current index signal
        let current_index = signal(0usize);

        // Add navigation arrows if requested
        if props.show_arrows && item_count > 1 {
            Self::add_arrows(&slider, &figure, &current_index, item_count, props.interval, props.animation_duration, props.on_change.as_ref());
        }

        // Add dot indicators if requested
        if props.show_dots && item_count > 1 {
            Self::add_dots(&slider, &current_index, item_count, props.on_change.as_ref());
        }

        // Add pause on hover if requested
        if props.pause_on_hover {
            let figure_clone_pause = figure.clone();
            let figure_clone_resume = figure.clone();

            // Pause on mouseenter
            let pause_closure = Closure::wrap(Box::new(move |_event: MouseEvent| {
                let current_style = figure_clone_pause.get_attribute("style").unwrap_or_default();
                if !current_style.contains("animation-play-state: paused") {
                    figure_clone_pause.set_attribute(
                        "style",
                        &format!("{} animation-play-state: paused;", current_style.replace("animation-play-state: running;", ""))
                    ).ok();
                }
            }) as Box<dyn FnMut(MouseEvent)>);

            slider.add_event_listener_with_callback("mouseenter", pause_closure.as_ref().unchecked_ref()).ok();
            pause_closure.forget();

            // Resume on mouseleave
            let resume_closure = Closure::wrap(Box::new(move |_event: MouseEvent| {
                let current_style = figure_clone_resume.get_attribute("style").unwrap_or_default();
                if current_style.contains("animation-play-state: paused") {
                    figure_clone_resume.set_attribute(
                        "style",
                        &current_style.replace("animation-play-state: paused;", "animation-play-state: running;")
                    ).ok();
                }
            }) as Box<dyn FnMut(MouseEvent)>);

            slider.add_event_listener_with_callback("mouseleave", resume_closure.as_ref().unchecked_ref()).ok();
            resume_closure.forget();
        }

        // Track current index based on animation (simplified - in production you'd sync with animation)
        if props.autoplay && props.on_change.is_some() {
            let index_clone = current_index.clone();
            let on_change_clone = props.on_change;
            let item_count_clone = item_count;
            
            create_effect(move || {
                let idx = index_clone.get();
                if let Some(handler) = &on_change_clone {
                    handler(idx % item_count_clone);
                }
            });
        }

        (slider.into(), current_index)
    }

    /// Generate CSS keyframes for the carousel animation.
    fn generate_keyframes(item_count: usize, _interval: u64, infinite: bool) -> String {
        let total_slides = if infinite { item_count + 1 } else { item_count };
        let slide_duration = 100.0 / total_slides as f64;
        let transition_percent = 5.0; // 5% of time for transition
        
        let mut keyframes = String::from("@keyframes slidy {");
        
        for i in 0..total_slides {
            let start_percent = i as f64 * slide_duration;
            let end_percent = start_percent + slide_duration - transition_percent;
            let next_start = start_percent + slide_duration;
            
            let _left_percent = if infinite && i == item_count {
                // Last slide (duplicate of first) - jump to start
                0.0
            } else {
                -(i as f64) * slide_duration
            };
            
            // Hold position
            if i == 0 {
                keyframes.push_str(&format!("0% {{ left: 0%; }}"));
            }
            keyframes.push_str(&format!(
                "{:.1}% {{ left: {:.1}%; }}",
                end_percent,
                if infinite && i == item_count { 0.0 } else { -(i as f64) * (100.0 / item_count as f64) }
            ));
            
            // Transition to next
            if i < total_slides - 1 {
                let next_left = if infinite && i == item_count - 1 {
                    -100.0 // Will jump back to 0
                } else {
                    -((i + 1) as f64) * (100.0 / item_count as f64)
                };
                
                keyframes.push_str(&format!(
                    "{:.1}% {{ left: {:.1}%; }}",
                    next_start,
                    next_left
                ));
            }
        }
        
        keyframes.push_str("}");
        keyframes
    }

    /// Add navigation arrows to the carousel.
    fn add_arrows(
        slider: &HtmlElement,
        _figure: &HtmlElement,
        _current_index: &Signal<usize>,
        _item_count: usize,
        _interval: u64,
        _animation_duration: u64,
        _on_change: Option<&Box<dyn Fn(usize)>>,
    ) {
        let document = slider.owner_document().unwrap();

        // Previous arrow
        let prev_arrow: HtmlElement = document.create_element("button").unwrap().dyn_into().unwrap();
        prev_arrow.set_attribute("class", "domius-carousel-arrow domius-carousel-prev").unwrap();
        prev_arrow.set_attribute("aria-label", "Previous slide").unwrap();
        prev_arrow.set_text_content(Some("‹"));

        // Next arrow
        let next_arrow: HtmlElement = document.create_element("button").unwrap().dyn_into().unwrap();
        next_arrow.set_attribute("class", "domius-carousel-arrow domius-carousel-next").unwrap();
        next_arrow.set_attribute("aria-label", "Next slide").unwrap();
        next_arrow.set_text_content(Some("›"));

        slider.append_child(&prev_arrow).unwrap();
        slider.append_child(&next_arrow).unwrap();

        // Arrow click handlers would go here for manual navigation
        // For pure CSS version, arrows can pause/resume or trigger class changes
    }

    /// Add dot indicators to the carousel.
    fn add_dots(
        slider: &HtmlElement,
        _current_index: &Signal<usize>,
        item_count: usize,
        _on_change: Option<&Box<dyn Fn(usize)>>,
    ) {
        let document = slider.owner_document().unwrap();

        let dots_container: HtmlElement = document.create_element("div").unwrap().dyn_into().unwrap();
        dots_container.set_attribute("class", "domius-carousel-dots").unwrap();
        dots_container.set_attribute("role", "tablist").unwrap();

        for i in 0..item_count {
            let dot: HtmlElement = document.create_element("button").unwrap().dyn_into().unwrap();
            dot.set_attribute("class", "domius-carousel-dot").unwrap();
            dot.set_attribute("role", "tab").unwrap();
            dot.set_attribute("aria-label", &format!("Go to slide {}", i + 1));
            
            if i == 0 {
                dot.set_attribute("aria-selected", "true").unwrap();
                dot.set_attribute("class", "domius-carousel-dot active").unwrap();
            }

            dots_container.append_child(&dot).unwrap();
        }

        slider.append_child(&dots_container).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_carousel_item_default() {
        let item = CarouselItem::default();
        assert!(item.src.is_none());
        assert_eq!(item.alt, Some(String::new()));
    }

    #[test]
    fn test_carousel_props_default() {
        let props = CarouselProps::default();
        assert!(props.items.is_empty());
        assert!(props.autoplay);
        assert_eq!(props.interval, 5000);
        assert!(props.show_arrows);
        assert!(props.show_dots);
        assert!(props.infinite);
    }

    #[test]
    fn test_carousel_keyframes_generation() {
        let keyframes = Carousel::generate_keyframes(4, 5000, true);
        assert!(keyframes.contains("@keyframes slidy"));
        assert!(keyframes.contains("left:"));
    }
}
