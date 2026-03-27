//! QRCode component for Domus.
//!
//! Generate and display QR codes.

use web_sys::Element;

/// QRCode error correction level.
#[derive(Clone, Copy, Default)]
pub enum QRCodeErrorLevel {
    #[default]
    Low,
    Medium,
    Quartile,
    High,
}

/// QRCode props.
#[derive(Clone, Default)]
pub struct QRCodeProps {
    /// Content to encode
    pub value: String,
    /// QRCode size in px
    pub size: u32,
    /// CSS class
    pub class: Option<String>,
    /// Error correction level
    pub error_level: QRCodeErrorLevel,
    /// Background color
    pub bg_color: Option<String>,
    /// Foreground color
    pub fg_color: Option<String>,
    /// Include margin
    pub include_margin: bool,
    /// Render as SVG
    pub svg: bool,
}

/// Build a QRCode component.
///
/// Note: This is a placeholder that generates a simple pattern.
/// For production use, integrate with a QR code generation library
/// like `qrcode` crate.
///
/// # Example
///
/// ```ignore
/// use domius_web::components::qrcode::{qrcode, QRCodeProps};
///
/// let qrcode_node = qrcode(QRCodeProps {
///     value: "https://example.com".to_string(),
///     size: 200,
///     ..Default::default()
/// });
/// ```
pub fn qrcode(props: QRCodeProps) -> Element {
    let document = web_sys::window().unwrap().document().unwrap();
    
    let container: Element = document.create_element("div").unwrap();
    
    let mut classes = String::from("qrcode");
    if let Some(class) = &props.class {
        classes.push_str(" ");
        classes.push_str(class);
    }
    container.set_class_name(&classes);

    // Set size
    let size_style = format!("width: {}px; height: {}px;", props.size, props.size);
    container.set_attribute("style", &size_style).ok();

    // Store value as data attribute for potential JS generation
    container.set_attribute("data-value", &props.value).ok();
    container.set_attribute("data-error-level", &match props.error_level {
        QRCodeErrorLevel::Low => "L",
        QRCodeErrorLevel::Medium => "M",
        QRCodeErrorLevel::Quartile => "Q",
        QRCodeErrorLevel::High => "H",
    }).ok();

    // Placeholder QR code pattern (for demo purposes)
    // In production, use a proper QR code generation library
    let canvas: Element = document.create_element("canvas").unwrap();
    canvas.set_class_name("qrcode-canvas");
    canvas.set_attribute("width", &props.size.to_string()).ok();
    canvas.set_attribute("height", &props.size.to_string()).ok();
    container.append_child(&canvas).unwrap();

    // Note for developers
    web_sys::console::warn_1(&"QRCode: For production, integrate with qrcode crate for proper generation".into());

    container
}
