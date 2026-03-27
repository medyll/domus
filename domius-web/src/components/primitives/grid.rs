//! Grid system for Domius.
//!
//! Responsive grid layout with Row and Col components.

use web_sys::Element;

/// Row props.
#[derive(Clone, Default)]
pub struct RowProps {
    /// Gutter between columns (in px)
    pub gutter: u32,
    /// CSS class
    pub class: Option<String>,
    /// Align (top, middle, bottom)
    pub align: Option<String>,
    /// Justify (start, end, center, space-around, space-between)
    pub justify: Option<String>,
}

/// Build a Row component.
pub fn row(props: RowProps) -> Element {
    let document = web_sys::window().unwrap().document().unwrap();
    let row: Element = document.create_element("div").unwrap();
    
    let mut classes = String::from("row");
    if let Some(class) = &props.class {
        classes.push_str(" ");
        classes.push_str(class);
    }
    row.set_class_name(&classes);

    if props.gutter > 0 {
        row.set_attribute("style", &format!("margin-left: -{}px; margin-right: -{}px;", props.gutter / 2, props.gutter / 2)).ok();
    }

    if let Some(align) = &props.align {
        row.set_attribute("data-align", align).ok();
    }
    if let Some(justify) = &props.justify {
        row.set_attribute("data-justify", justify).ok();
    }

    row
}

/// Col props.
#[derive(Clone, Default)]
pub struct ColProps {
    /// Span on extra small screens (<576px)
    pub xs: Option<u32>,
    /// Span on small screens (≥576px)
    pub sm: Option<u32>,
    /// Span on medium screens (≥768px)
    pub md: Option<u32>,
    /// Span on large screens (≥992px)
    pub lg: Option<u32>,
    /// Span on extra large screens (≥1200px)
    pub xl: Option<u32>,
    /// Span on all screens (default)
    pub span: Option<u32>,
    /// Offset on all screens
    pub offset: Option<u32>,
    /// CSS class
    pub class: Option<String>,
}

/// Build a Col component.
pub fn col(props: ColProps) -> Element {
    let document = web_sys::window().unwrap().document().unwrap();
    let col: Element = document.create_element("div").unwrap();
    
    let mut classes = String::from("col");
    
    if let Some(span) = props.span {
        classes.push_str(&format!(" col-{}", span));
    }
    if let Some(xs) = props.xs {
        classes.push_str(&format!(" col-xs-{}", xs));
    }
    if let Some(sm) = props.sm {
        classes.push_str(&format!(" col-sm-{}", sm));
    }
    if let Some(md) = props.md {
        classes.push_str(&format!(" col-md-{}", md));
    }
    if let Some(lg) = props.lg {
        classes.push_str(&format!(" col-lg-{}", lg));
    }
    if let Some(xl) = props.xl {
        classes.push_str(&format!(" col-xl-{}", xl));
    }
    if let Some(offset) = props.offset {
        classes.push_str(&format!(" offset-{}", offset));
    }
    if let Some(class) = &props.class {
        classes.push_str(" ");
        classes.push_str(class);
    }
    
    col.set_class_name(&classes);
    col
}
