//! QRCode component for Domus.
//!
//! Generate and display QR codes.

use qrcode::{Color, EcLevel, QrCode};
use web_sys::{Document, Element};

/// Modules of quiet zone required around a QR code for reliable decoding.
const QUIET_ZONE: u32 = 4;

/// Fallback edge length, in pixels, of a single module when no size is given.
const DEFAULT_MODULE_PIXELS: u32 = 6;

/// QRCode error correction level.
#[derive(Clone, Copy, Default)]
pub enum QRCodeErrorLevel {
    #[default]
    Low,
    Medium,
    Quartile,
    High,
}

impl QRCodeErrorLevel {
    fn token(self) -> &'static str {
        match self {
            Self::Low => "L",
            Self::Medium => "M",
            Self::Quartile => "Q",
            Self::High => "H",
        }
    }

    fn level(self) -> EcLevel {
        match self {
            Self::Low => EcLevel::L,
            Self::Medium => EcLevel::M,
            Self::Quartile => EcLevel::Q,
            Self::High => EcLevel::H,
        }
    }
}

/// The dark and light grid of an encoded value, quiet zone included.
///
/// This is the whole of the QR code; drawing it is a separate concern, which
/// keeps the encoding testable without a browser.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QRCodeMatrix {
    /// Side length of the code itself, in modules.
    pub modules: u32,
    /// Modules of quiet zone kept on each side.
    pub quiet: u32,
    dark: Vec<bool>,
}

impl QRCodeMatrix {
    /// Side length of the drawn grid, quiet zone included.
    pub fn extent(&self) -> u32 {
        self.modules + self.quiet * 2
    }

    /// Whether the module at `(x, y)` of the drawn grid is dark.
    pub fn is_dark(&self, x: u32, y: u32) -> bool {
        if x < self.quiet || y < self.quiet {
            return false;
        }
        let (x, y) = (x - self.quiet, y - self.quiet);
        if x >= self.modules || y >= self.modules {
            return false;
        }
        self.dark[(y * self.modules + x) as usize]
    }

    /// Coordinates of every dark module in the drawn grid.
    pub fn dark_modules(&self) -> impl Iterator<Item = (u32, u32)> + '_ {
        let modules = self.modules;
        let quiet = self.quiet;
        self.dark
            .iter()
            .enumerate()
            .filter(|(_, dark)| **dark)
            .map(move |(index, _)| {
                let index = index as u32;
                (index % modules + quiet, index / modules + quiet)
            })
    }
}

/// Encode `value` into a QR matrix, or explain why it does not fit.
pub fn qrcode_matrix(
    value: &str,
    error_level: QRCodeErrorLevel,
    include_margin: bool,
) -> Result<QRCodeMatrix, String> {
    let code = QrCode::with_error_correction_level(value.as_bytes(), error_level.level())
        .map_err(|error| error.to_string())?;
    let modules = code.width() as u32;
    Ok(QRCodeMatrix {
        modules,
        quiet: if include_margin { QUIET_ZONE } else { 0 },
        dark: code
            .into_colors()
            .into_iter()
            .map(|color| color == Color::Dark)
            .collect(),
    })
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
}

/// Build a QRCode component.
///
/// The value is encoded for real and drawn as one SVG rect per dark module, so
/// the rendered code decodes back to `value`. Set `include_margin` to keep the
/// four-module quiet zone that most scanners expect.
///
/// A value too long for any QR version yields an element carrying `data-error`
/// and no modules, rather than a panic on a visible path.
///
/// # Example
///
/// ```ignore
/// use domius_web::components::primitives::qrcode::{qrcode, QRCodeProps};
///
/// let qrcode_node = qrcode(QRCodeProps {
///     value: "https://example.com".to_string(),
///     size: 200,
///     include_margin: true,
///     ..Default::default()
/// });
/// ```
pub fn qrcode(props: QRCodeProps) -> Element {
    let document = web_sys::window()
        .expect("no window")
        .document()
        .expect("no document");
    let container = document
        .create_element("div")
        .expect("create qrcode container");
    let mut classes = vec!["qrcode"];
    if let Some(class) = props.class.as_deref() {
        classes.push(class);
    }
    container.set_class_name(&classes.join(" "));
    container
        .set_attribute("data-value", &props.value)
        .expect("set qrcode value");
    container
        .set_attribute("data-error-level", props.error_level.token())
        .expect("set qrcode error level");

    let matrix = match qrcode_matrix(&props.value, props.error_level, props.include_margin) {
        Ok(matrix) => matrix,
        Err(error) => {
            container
                .set_attribute("data-error", &error)
                .expect("report qrcode failure");
            return container;
        }
    };

    let extent = matrix.extent();
    let pixels = if props.size == 0 {
        extent * DEFAULT_MODULE_PIXELS
    } else {
        props.size
    };
    container
        .set_attribute("data-modules", &matrix.modules.to_string())
        .expect("set qrcode module count");

    let svg = svg_element(&document, "svg");
    svg.set_attribute("viewBox", &format!("0 0 {extent} {extent}"))
        .expect("set qrcode view box");
    svg.set_attribute("width", &pixels.to_string())
        .expect("size qrcode");
    svg.set_attribute("height", &pixels.to_string())
        .expect("size qrcode");
    svg.set_attribute("role", "img").expect("set qrcode role");
    svg.set_attribute("aria-label", &format!("QR code for {}", props.value))
        .expect("label qrcode");
    svg.set_attribute("data-quiet-zone", &matrix.quiet.to_string())
        .expect("set qrcode quiet zone");
    // Scanners need the light ground as much as the dark modules.
    svg.set_attribute("shape-rendering", "crispEdges")
        .expect("keep qrcode module edges sharp");

    let background = svg_element(&document, "rect");
    background
        .set_attribute("data-role", "background")
        .expect("mark qrcode background");
    background
        .set_attribute("width", "100%")
        .expect("stretch qrcode background");
    background
        .set_attribute("height", "100%")
        .expect("stretch qrcode background");
    background
        .set_attribute("fill", props.bg_color.as_deref().unwrap_or("#ffffff"))
        .expect("colour qrcode background");
    svg.append_child(&background)
        .expect("append qrcode background");

    let group = svg_element(&document, "g");
    group
        .set_attribute("data-role", "modules")
        .expect("mark qrcode modules");
    group
        .set_attribute("fill", props.fg_color.as_deref().unwrap_or("#000000"))
        .expect("colour qrcode modules");
    for (x, y) in matrix.dark_modules() {
        let module = svg_element(&document, "rect");
        module
            .set_attribute("x", &x.to_string())
            .expect("position qrcode module");
        module
            .set_attribute("y", &y.to_string())
            .expect("position qrcode module");
        module
            .set_attribute("width", "1")
            .expect("size qrcode module");
        module
            .set_attribute("height", "1")
            .expect("size qrcode module");
        group.append_child(&module).expect("append qrcode module");
    }
    svg.append_child(&group).expect("append qrcode modules");
    container.append_child(&svg).expect("append qrcode graphic");
    container
}

fn svg_element(document: &Document, tag: &str) -> Element {
    document
        .create_element_ns(Some("http://www.w3.org/2000/svg"), tag)
        .expect("create SVG element")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Blow the matrix up to a bitmap a real detector can work on.
    fn rasterize(matrix: &QRCodeMatrix, scale: u32) -> (usize, usize, Vec<u8>) {
        let side = (matrix.extent() * scale) as usize;
        let mut pixels = vec![255u8; side * side];
        for (x, y) in matrix.dark_modules() {
            for row in 0..scale {
                for column in 0..scale {
                    let py = (y * scale + row) as usize;
                    let px = (x * scale + column) as usize;
                    pixels[py * side + px] = 0;
                }
            }
        }
        (side, side, pixels)
    }

    fn decode(matrix: &QRCodeMatrix) -> String {
        let (width, height, pixels) = rasterize(matrix, 8);
        let mut image = rqrr::PreparedImage::prepare_from_greyscale(width, height, |x, y| {
            pixels[y * width + x]
        });
        let grids = image.detect_grids();
        assert_eq!(grids.len(), 1, "exactly one QR code should be detected");
        grids[0].decode().expect("the grid should decode").1
    }

    #[test]
    fn an_encoded_url_decodes_back_to_itself() {
        let url = "http://127.0.0.1:8080/reports";
        let matrix = qrcode_matrix(url, QRCodeErrorLevel::Medium, true).expect("url should encode");

        assert_eq!(matrix.quiet, QUIET_ZONE);
        assert_eq!(matrix.extent(), matrix.modules + QUIET_ZONE * 2);
        assert_eq!(decode(&matrix), url);
    }

    #[test]
    fn every_error_level_still_decodes() {
        let value = "https://domius.local/reports?window=60m";
        for level in [
            QRCodeErrorLevel::Low,
            QRCodeErrorLevel::Medium,
            QRCodeErrorLevel::Quartile,
            QRCodeErrorLevel::High,
        ] {
            let matrix = qrcode_matrix(value, level, true).expect("value should encode");
            assert_eq!(decode(&matrix), value, "level {} failed", level.token());
        }
    }

    #[test]
    fn the_quiet_zone_offsets_every_module_without_changing_the_code() {
        let value = "domius";
        let bare = qrcode_matrix(value, QRCodeErrorLevel::Low, false).expect("value should encode");
        let padded =
            qrcode_matrix(value, QRCodeErrorLevel::Low, true).expect("value should encode");

        assert_eq!(bare.quiet, 0);
        assert_eq!(bare.extent(), bare.modules);
        assert_eq!(padded.extent(), bare.modules + QUIET_ZONE * 2);

        let bare_modules = bare.dark_modules().collect::<Vec<_>>();
        let padded_modules = padded
            .dark_modules()
            .map(|(x, y)| (x - QUIET_ZONE, y - QUIET_ZONE))
            .collect::<Vec<_>>();
        assert_eq!(bare_modules, padded_modules);
        assert!(!bare_modules.is_empty());

        // The quiet zone itself stays light on every side.
        for offset in 0..padded.extent() {
            assert!(!padded.is_dark(offset, 0));
            assert!(!padded.is_dark(offset, padded.extent() - 1));
            assert!(!padded.is_dark(0, offset));
            assert!(!padded.is_dark(padded.extent() - 1, offset));
        }
    }

    #[test]
    fn a_value_too_long_for_any_version_is_reported_not_panicked() {
        let overflowing = "x".repeat(8000);
        assert!(qrcode_matrix(&overflowing, QRCodeErrorLevel::High, true).is_err());
    }
}
