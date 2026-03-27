//! Avatar component - User profile image display.

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, HtmlImageElement};

/// Avatar size.
#[derive(Clone, PartialEq, Debug)]
pub enum AvatarSize {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

impl Default for AvatarSize {
    fn default() -> Self {
        Self::Md
    }
}

/// Avatar shape.
#[derive(Clone, PartialEq, Debug)]
pub enum AvatarShape {
    Circle,
    Square,
    Rounded,
}

impl Default for AvatarShape {
    fn default() -> Self {
        Self::Circle
    }
}

/// Props for the Avatar component.
#[derive(Clone)]
pub struct AvatarProps {
    pub src: Option<String>,
    pub alt: Option<String>,
    pub name: Option<String>,
    pub size: AvatarSize,
    pub shape: AvatarShape,
    pub class: Option<String>,
}

impl Default for AvatarProps {
    fn default() -> Self {
        Self {
            src: None,
            alt: None,
            name: None,
            size: AvatarSize::default(),
            shape: AvatarShape::default(),
            class: None,
        }
    }
}

/// Avatar component.
pub struct Avatar;

impl Avatar {
    /// Create an avatar element.
    pub fn create(props: AvatarProps) -> Element {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");

        let avatar: HtmlElement = document
            .create_element("div")
            .unwrap()
            .dyn_into()
            .unwrap();

        // Build class names
        let mut classes = vec![
            "domius-avatar".to_string(),
            format!("domius-avatar-{:?}", props.size).to_lowercase(),
            format!("domius-avatar-{:?}", props.shape).to_lowercase(),
        ];
        if let Some(class) = &props.class {
            classes.push(class.clone());
        }
        avatar.set_attribute("class", &classes.join(" ")).unwrap();

        // Add image if src provided
        if let Some(src) = &props.src {
            let img: HtmlImageElement = document
                .create_element("img")
                .unwrap()
                .dyn_into()
                .unwrap();
            img.set_src(src);
            img.set_attribute("class", "domius-avatar-image").unwrap();
            if let Some(alt) = &props.alt {
                img.set_alt(alt);
            }
            avatar.append_child(&img).unwrap();
        } else if let Some(name) = &props.name {
            // Show initials
            let initials = name
                .split_whitespace()
                .take(2)
                .filter_map(|word| word.chars().next())
                .collect::<String>()
                .to_uppercase();

            let initials_el: HtmlElement = document
                .create_element("span")
                .unwrap()
                .dyn_into()
                .unwrap();
            initials_el.set_attribute("class", "domius-avatar-initials").unwrap();
            initials_el.set_text_content(Some(&initials));
            avatar.append_child(&initials_el).unwrap();
        }

        avatar.into()
    }
}

/// Props for AvatarGroup component.
#[derive(Clone)]
pub struct AvatarGroupProps {
    pub avatars: Vec<AvatarProps>,
    pub max_visible: usize,
    pub size: AvatarSize,
    pub overlap: bool,
    pub class: Option<String>,
}

impl Default for AvatarGroupProps {
    fn default() -> Self {
        Self {
            avatars: Vec::new(),
            max_visible: 4,
            size: AvatarSize::default(),
            overlap: true,
            class: None,
        }
    }
}

/// AvatarGroup component - Stack of avatars.
pub struct AvatarGroup;

impl AvatarGroup {
    /// Create an avatar group element.
    pub fn create(props: AvatarGroupProps) -> Element {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");

        let group: HtmlElement = document
            .create_element("div")
            .unwrap()
            .dyn_into()
            .unwrap();
        group.set_attribute("class", "domius-avatar-group").unwrap();

        let visible_count = props.avatars.len().min(props.max_visible);
        let remaining = props.avatars.len().saturating_sub(props.max_visible);

        for (i, avatar_props) in props.avatars.iter().take(visible_count).enumerate() {
            let mut props_clone = avatar_props.clone();
            props_clone.size = props.size.clone();
            let avatar = Avatar::create(props_clone);
            group.append_child(&avatar).unwrap();
        }

        // Add "+N" indicator for remaining avatars
        if remaining > 0 {
            let remaining_el: HtmlElement = document
                .create_element("div")
                .unwrap()
                .dyn_into()
                .unwrap();
            remaining_el.set_attribute("class", "domius-avatar-remaining").unwrap();
            remaining_el.set_text_content(Some(&format!("+{}", remaining)));
            group.append_child(&remaining_el).unwrap();
        }

        group.into()
    }
}
