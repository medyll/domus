//! Icon component - SVG icon display component.

use domius_core::signal::Signal;
use wasm_bindgen::prelude::*;
use web_sys::{Element, SvgElement};

use crate::component::DomiusNode;

/// Predefined icon names.
///
/// These are common icons that can be rendered as SVG.
/// For a production app, you would typically use an icon font or sprite.
#[derive(Clone, PartialEq, Debug)]
pub enum IconName {
    // Navigation
    Menu,
    Close,
    ChevronLeft,
    ChevronRight,
    ChevronUp,
    ChevronDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,
    
    // Actions
    Search,
    Edit,
    Delete,
    Add,
    Remove,
    Check,
    Clear,
    Save,
    Cancel,
    Refresh,
    Settings,
    MoreVert,
    MoreHoriz,
    
    // Status
    Info,
    Warning,
    Error,
    Success,
    Help,
    
    // Media
    Play,
    Pause,
    Stop,
    VolumeUp,
    VolumeOff,
    Image,
    
    // Content
    File,
    Folder,
    Download,
    Upload,
    Cloud,
    
    // Social
    Share,
    Link,
    Favorite,
    FavoriteBorder,
    
    // UI
    Home,
    User,
    Account,
    Logout,
    Login,
    Notifications,
    Calendar,
    Clock,
    Visibility,
    VisibilityOff,
    Lock,
    LockOpen,
    
    // Forms
    Star,
    StarBorder,
    Rating,
}

impl IconName {
    /// Get the SVG path data for this icon.
    pub fn path_data(&self) -> &'static str {
        match self {
            // Navigation
            IconName::Menu => "M3 18h18v-2H3v2zm0-5h18v-2H3v2zm0-7v2h18V6H3z",
            IconName::Close => "M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z",
            IconName::ChevronLeft => "M15.41 7.41L14 6l-6 6 6 6 1.41-1.41L10.83 12z",
            IconName::ChevronRight => "M10 6L8.59 7.41 13.17 12l-4.58 4.59L10 18l6-6z",
            IconName::ChevronUp => "M12 8l-6 6 1.41 1.41L12 10.83l4.59 4.58L18 14z",
            IconName::ChevronDown => "M16.59 8.59L12 13.17 7.41 8.59 6 10l6 6 6-6z",
            IconName::ArrowLeft => "M14 7l-5 5 5 5V7z",
            IconName::ArrowRight => "M10 17l5-5-5-5v10z",
            IconName::ArrowUp => "M4 12l1.41 1.41L11 7.83V20h2V7.83l5.58 5.59L20 12l-8-8-8 8z",
            IconName::ArrowDown => "M20 12l-1.41-1.41L13 16.17V4h-2v12.17l-5.58-5.59L4 12l8 8 8-8z",
            
            // Actions
            IconName::Search => "M15.5 14h-.79l-.28-.27C15.41 12.59 16 11.11 16 9.5 16 5.91 13.09 3 9.5 3S3 5.91 3 9.5 5.91 16 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z",
            IconName::Edit => "M3 17.25V21h3.75L17.81 9.94l-3.75-3.75L3 17.25zM20.71 7.04c.39-.39.39-1.02 0-1.41l-2.34-2.34c-.39-.39-1.02-.39-1.41 0l-1.83 1.83 3.75 3.75 1.83-1.83z",
            IconName::Delete => "M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z",
            IconName::Add => "M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z",
            IconName::Remove => "M19 13H5v-2h14v2z",
            IconName::Check => "M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z",
            IconName::Clear => "M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z",
            IconName::Save => "M17 3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V7l-4-4zm-5 16c-1.66 0-3-1.34-3-3s1.34-3 3-3 3 1.34 3 3-1.34 3-3 3zm3-10H5V5h10v4z",
            IconName::Cancel => "M12 2C6.47 2 2 6.47 2 12s4.47 10 10 10 10-4.47 10-10S17.53 2 12 2zm5 13.59L15.59 17 12 13.41 8.41 17 7 15.59 10.59 12 7 8.41 8.41 7 12 10.59 15.59 7 17 8.41 13.41 12 17 15.59z",
            IconName::Refresh => "M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z",
            IconName::Settings => "M19.14 12.94c.04-.31.06-.63.06-.94 0-.31-.02-.63-.06-.94l2.03-1.58c.18-.14.23-.41.12-.61l-1.92-3.32c-.12-.22-.37-.29-.59-.22l-2.39.96c-.5-.38-1.03-.7-1.62-.94l-.36-2.54c-.04-.24-.24-.41-.48-.41h-3.84c-.24 0-.43.17-.47.41l-.36 2.54c-.59.24-1.13.57-1.62.94l-2.39-.96c-.22-.08-.47 0-.59.22L5.09 8.87c-.12.21-.08.47.12.61l2.03 1.58c-.04.31-.06.63-.06.94s.02.63.06.94l-2.03 1.58c-.18.14-.23.41-.12.61l1.92 3.32c.12.22.37.29.59.22l2.39-.96c.5.38 1.03.7 1.62.94l.36 2.54c.05.24.24.41.48.41h3.84c.24 0 .44-.17.47-.41l.36-2.54c.59-.24 1.13-.56 1.62-.94l2.39.96c.22.08.47 0 .59-.22l1.92-3.32c.12-.22.07-.47-.12-.61l-2.01-1.58zM12 15.6c-1.98 0-3.6-1.62-3.6-3.6s1.62-3.6 3.6-3.6 3.6 1.62 3.6 3.6-1.62 3.6-3.6 3.6z",
            IconName::MoreVert => "M12 8c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm0 2c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0 6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z",
            IconName::MoreHoriz => "M6 10c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm12 0c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm-6 0c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z",
            
            // Status
            IconName::Info => "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-6h2v6zm0-8h-2V7h2v2z",
            IconName::Warning => "M1 21h22L12 2 1 21zm12-3h-2v-2h2v2zm0-4h-2v-4h2v4z",
            IconName::Error => "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-2h2v2zm0-4h-2V7h2v6z",
            IconName::Success => "M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z",
            IconName::Help => "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 17h-2v-2h2v2zm2.07-7.75l-.9.92C13.45 12.9 13 13.5 13 15h-2v-.5c0-1.1.45-2.1 1.17-2.83l1.24-1.26c.37-.36.59-.86.59-1.41 0-1.1-.9-2-2-2s-2 .9-2 2H8c0-2.21 1.79-4 4-4s4 1.79 4 4c0 .88-.36 1.68-.93 2.25z",
            
            // Media
            IconName::Play => "M8 5v14l11-7z",
            IconName::Pause => "M6 19h4V5H6v14zm8-14v14h4V5h-4z",
            IconName::Stop => "M6 6h12v12H6z",
            IconName::VolumeUp => "M3 9v6h4l5 5V4L7 9H3zm13.5 3c0-1.77-1.02-3.29-2.5-4.03v8.05c1.48-.73 2.5-2.25 2.5-4.02zM14 3.23v2.06c2.89.86 5 3.54 5 6.71s-2.11 5.85-5 6.71v2.06c4.01-.91 7-4.49 7-8.77s-2.99-7.86-7-8.77z",
            IconName::VolumeOff => "M16.5 12c0-1.77-1.02-3.29-2.5-4.03v2.21l2.45 2.45c.03-.2.05-.41.05-.63zm2.5 0c0 .94-.2 1.82-.54 2.64l1.51 1.51C20.63 14.91 21 13.5 21 12c0-4.28-2.99-7.86-7-8.77v2.06c2.89.86 5 3.54 5 6.71zM4.27 3L3 4.27 7.73 9H3v6h4l5 5v-6.73l4.25 4.25c-.67.52-1.42.93-2.25 1.18v2.06c1.38-.31 2.63-.95 3.69-1.81L19.73 21 21 19.73l-9-9L4.27 3zM12 4L9.91 6.09 12 8.18V4z",
            IconName::Image => "M21 19V5c0-1.1-.9-2-2-2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2zM8.5 13.5l2.5 3.01L14.5 12l4.5 6H5l3.5-4.5z",
            
            // Content
            IconName::File => "M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z",
            IconName::Folder => "M10 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2h-8l-2-2z",
            IconName::Download => "M19 9h-4V3H9v6H5l7 7 7-7zM5 18v2h14v-2H5z",
            IconName::Upload => "M9 16h6v-6h4l-7-7-7 7h4zm-4 2h14v2H5z",
            IconName::Cloud => "M19.35 10.04C18.67 6.59 15.64 4 12 4 9.11 4 6.6 5.64 5.35 8.04 2.34 8.36 0 10.91 0 14c0 3.31 2.69 6 6 6h13c2.76 0 5-2.24 5-5 0-2.64-2.05-4.78-4.65-4.96z",
            
            // Social
            IconName::Share => "M18 16.08c-.76 0-1.44.3-1.96.77L8.91 12.7c.05-.23.09-.46.09-.7s-.04-.47-.09-.7l7.05-4.11c.54.5 1.25.81 2.04.81 1.66 0 3-1.34 3-3s-1.34-3-3-3-3 1.34-3 3c0 .24.04.47.09.7L8.04 9.81C7.5 9.31 6.79 9 6 9c-1.66 0-3 1.34-3 3s1.34 3 3 3c.79 0 1.5-.31 2.04-.81l7.12 4.16c-.05.21-.08.43-.08.65 0 1.61 1.31 2.92 2.92 2.92s2.92-1.31 2.92-2.92-1.31-2.92-2.92-2.92z",
            IconName::Link => "M3.9 12c0-1.71 1.39-3.1 3.1-3.1h4V7H7c-2.76 0-5 2.24-5 5s2.24 5 5 5h4v-1.9H7c-1.71 0-3.1-1.39-3.1-3.1zM8 13h8v-2H8v2zm9-6h-4v1.9h4c1.71 0 3.1 1.39 3.1 3.1s-1.39 3.1-3.1 3.1h-4V17h4c2.76 0 5-2.24 5-5s-2.24-5-5-5z",
            IconName::Favorite => "M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z",
            IconName::FavoriteBorder => "M16.5 3c-1.74 0-3.41.81-4.5 2.09C10.91 3.81 9.24 3 7.5 3 4.42 3 2 5.42 2 8.5c0 3.78 3.4 6.86 8.55 11.54L12 21.35l1.45-1.32C18.6 15.36 22 12.28 22 8.5 22 5.42 19.58 3 16.5 3zm-4.4 15.55l-.1.1-.1-.1C7.14 14.24 4 11.39 4 8.5 4 6.5 5.5 5 7.5 5c1.54 0 3.04.99 3.56 2.36h1.87C13.46 5.99 14.96 5 16.5 5c2 0 3.5 1.5 3.5 3.5 0 2.89-3.14 5.74-7.9 10.05z",
            
            // UI
            IconName::Home => "M10 20v-6h4v6h5v-8h3L12 3 2 12h3v8z",
            IconName::User => "M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z",
            IconName::Account => "M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z",
            IconName::Logout => "M17 7l-1.41 1.41L18.17 11H8v2h10.17l-2.58 2.58L17 17l5-5zM4 5h8V3H4c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h8v-2H4V5z",
            IconName::Login => "M11 7L9.59 8.41 13.17 12H8v2h5.17l-3.58 3.59L11 19l5-5zM4 5h8V3H4c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h8v-2H4V5z",
            IconName::Notifications => "M12 22c1.1 0 2-.9 2-2h-4c0 1.1.89 2 2 2zm6-6v-5c0-3.07-1.64-5.64-4.5-6.32V4c0-.83-.67-1.5-1.5-1.5s-1.5.67-1.5 1.5v.68C7.63 5.36 6 7.92 6 11v5l-2 2v1h16v-1l-2-2z",
            IconName::Calendar => "M19 3h-1V1h-2v2H8V1H6v2H5c-1.11 0-1.99.9-1.99 2L3 19c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V8h14v11zM7 10h5v5H7z",
            IconName::Clock => "M11.99 2C6.47 2 2 6.48 2 12s4.47 10 10 10 10-4.48 10-10S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8zm.5-13H11v6l5.25 3.15.75-1.23-4.5-2.67z",
            IconName::Visibility => "M12 4.5C7 4.5 2.73 7.61 1 12c1.73 4.39 6 7.5 11 7.5s9.27-3.11 11-7.5c-1.73-4.39-6-7.5-11-7.5zM12 17c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5zm0-8c-1.66 0-3 1.34-3 3s1.34 3 3 3 3-1.34 3-3-1.34-3-3-3z",
            IconName::VisibilityOff => "M12 7c2.76 0 5 2.24 5 5 0 .65-.13 1.26-.36 1.83l2.92 2.92c1.51-1.26 2.7-2.89 3.43-4.75-1.73-4.39-6-7.5-11-7.5-1.4 0-2.74.25-3.98.7l2.16 2.16C10.74 7.13 11.35 7 12 7zM2 4.27l2.28 2.28.46.46C3.08 8.3 1.78 10.02 1 12c1.73 4.39 6 7.5 11 7.5 1.55 0 3.03-.3 4.38-.84l.42.42L19.73 22 21 20.73 3.27 3 2 4.27zM7.53 9.8l1.55 1.55c-.05.21-.08.43-.08.65 0 1.66 1.34 3 3 3 .22 0 .44-.03.65-.08l1.55 1.55c-.67.33-1.41.53-2.2.53-2.76 0-5-2.24-5-5 0-.79.2-1.53.53-2.2zm4.31-.78l3.15 3.15.02-.16c0-1.66-1.34-3-3-3l-.17.01z",
            IconName::Lock => "M18 8h-1V6c0-2.76-2.24-5-5-5S7 3.24 7 6v2H6c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V10c0-1.1-.9-2-2-2zm-6 9c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2zm3.1-9H8.9V6c0-1.71 1.39-3.1 3.1-3.1 1.71 0 3.1 1.39 3.1 3.1v2z",
            IconName::LockOpen => "M12 17c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm6-9h-1V6c0-2.76-2.24-5-5-5S7 3.24 7 6h1.9c0-1.71 1.39-3.1 3.1-3.1 1.71 0 3.1 1.39 3.1 3.1v2H6c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V10c0-1.1-.9-2-2-2zm0 12H6V10h12v10z",
            
            // Forms
            IconName::Star => "M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21z",
            IconName::StarBorder => "M22 9.24l-7.19-.62L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21 12 17.27 18.18 21l-1.63-7.03L22 9.24zM12 15.4l-3.76 2.27 1-4.28-3.32-2.88 4.38-.38L12 6.1l1.71 4.01 4.38.38-3.32 2.88 1 4.28L12 15.4z",
            IconName::Rating => "M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21z",
        }
    }

    /// Get the viewBox for this icon (usually "0 0 24 24").
    pub fn view_box(&self) -> &'static str {
        "0 0 24 24"
    }
}

/// Props for the Icon component.
pub struct IconProps {
    /// Icon to display
    pub icon: IconName,
    /// Icon size in pixels
    pub size: u32,
    /// Icon color (CSS color value)
    pub color: Option<String>,
    /// Additional CSS classes
    pub class: Option<String>,
    /// Icon title for accessibility
    pub title: Option<String>,
    /// Click handler
    pub on_click: Option<Box<dyn Fn()>>,
    /// Whether the icon is clickable (adds cursor pointer)
    pub clickable: bool,
}

impl Default for IconProps {
    fn default() -> Self {
        Self {
            icon: IconName::Info,
            size: 24,
            color: None,
            class: None,
            title: None,
            on_click: None,
            clickable: false,
        }
    }
}

/// Icon component.
pub struct Icon;

impl Icon {
    /// Create an SVG icon element.
    pub fn create(props: IconProps) -> Element {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");

        // Create SVG element
        let svg = document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")
            .unwrap();

        // Set attributes
        svg.set_attribute("viewBox", props.icon.view_box()).unwrap();
        svg.set_attribute("width", &props.size.to_string()).unwrap();
        svg.set_attribute("height", &props.size.to_string()).unwrap();
        svg.set_attribute("fill", "currentColor").unwrap();
        
        // Build class names
        let mut classes = vec!["domius-icon".to_string()];
        if props.clickable {
            classes.push("domius-icon-clickable".to_string());
        }
        if let Some(class) = &props.class {
            classes.push(class.clone());
        }
        svg.set_attribute("class", &classes.join(" ")).unwrap();

        // Set color if provided
        if let Some(color) = &props.color {
            svg.set_attribute("style", &format!("color: {}", color)).unwrap();
        }

        // Add title for accessibility
        if let Some(title) = &props.title {
            let title_el = document
                .create_element_ns(Some("http://www.w3.org/2000/svg"), "title")
                .unwrap();
            title_el.set_text_content(Some(title));
            svg.append_child(&title_el).unwrap();
        }

        // Create path element
        let path = document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "path")
            .unwrap();
        path.set_attribute("d", props.icon.path_data()).unwrap();
        path.set_attribute("fill", "currentColor").unwrap();
        
        svg.append_child(&path).unwrap();

        // Attach click handler if provided
        if let Some(handler) = props.on_click {
            use wasm_bindgen::JsCast;
            let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
                handler();
            }) as Box<dyn FnMut(web_sys::MouseEvent)>);
            
            svg.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
        }

        svg.into()
    }

    /// Create an icon with just the icon name.
    pub fn new(icon: IconName) -> Element {
        Self::create(IconProps {
            icon,
            ..Default::default()
        })
    }

    /// Create an icon with a specific size.
    pub fn sized(icon: IconName, size: u32) -> Element {
        Self::create(IconProps {
            icon,
            size,
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_path_data() {
        // Just verify we get non-empty strings
        assert!(!IconName::Menu.path_data().is_empty());
        assert!(!IconName::Close.path_data().is_empty());
        assert!(!IconName::Check.path_data().is_empty());
    }

    #[test]
    fn test_icon_view_box() {
        assert_eq!(IconName::Menu.view_box(), "0 0 24 24");
    }

    #[test]
    fn test_icon_props_default() {
        let props = IconProps::default();
        assert_eq!(props.size, 24);
        assert_eq!(props.icon, IconName::Info);
        assert!(props.color.is_none());
    }
}
