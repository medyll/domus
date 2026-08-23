//! Typography component for Domius.
//!
//! Text styling components: Title, Paragraph, Text, Link.

use web_sys::Element;

/// Title level.
#[derive(Clone, Copy, Default)]
pub enum TitleLevel {
    #[default]
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

/// Title props.
#[derive(Clone, Default)]
pub struct TitleProps {
    /// Heading level
    pub level: TitleLevel,
    /// Title text
    pub text: String,
    /// CSS class
    pub class: Option<String>,
    /// Muted style
    pub muted: bool,
    /// Mark (highlight) style
    pub mark: bool,
    /// Code style
    pub code: bool,
    /// Delete (strikethrough) style
    pub delete: bool,
    /// Underline style
    pub underline: bool,
    /// Strong (bold) style
    pub strong: bool,
    /// Ellipsis for overflow
    pub ellipsis: bool,
    /// Copyable (show copy icon)
    pub copyable: bool,
}

/// Build a Title component.
pub fn title(props: TitleProps) -> Element {
    let document = web_sys::window().unwrap().document().unwrap();

    let tag = match props.level {
        TitleLevel::H1 => "h1",
        TitleLevel::H2 => "h2",
        TitleLevel::H3 => "h3",
        TitleLevel::H4 => "h4",
        TitleLevel::H5 => "h5",
        TitleLevel::H6 => "h6",
    };

    let title: Element = document.create_element(tag).unwrap();

    let mut classes = String::from("title");
    if props.muted {
        classes.push_str(" title-muted");
    }
    if let Some(class) = &props.class {
        classes.push_str(" ");
        classes.push_str(class);
    }
    title.set_class_name(&classes);

    // Build content with modifiers
    let mut content = props.text.clone();

    if props.mark {
        let mark_el: Element = document.create_element("mark").unwrap();
        mark_el.set_text_content(Some(&content));
        content = String::new();
        title.append_child(&mark_el).unwrap();
    }

    if props.code {
        let code_el: Element = document.create_element("code").unwrap();
        code_el.set_text_content(Some(&content));
        content = String::new();
        title.append_child(&code_el).unwrap();
    }

    if props.delete {
        let del_el: Element = document.create_element("del").unwrap();
        del_el.set_text_content(Some(&content));
        content = String::new();
        title.append_child(&del_el).unwrap();
    }

    if props.underline {
        let u_el: Element = document.create_element("u").unwrap();
        u_el.set_text_content(Some(&content));
        content = String::new();
        title.append_child(&u_el).unwrap();
    }

    if props.strong {
        let strong_el: Element = document.create_element("strong").unwrap();
        strong_el.set_text_content(Some(&content));
        content = String::new();
        title.append_child(&strong_el).unwrap();
    }

    if !content.is_empty() {
        title
            .append_child(&document.create_text_node(&content))
            .unwrap();
    }

    if props.ellipsis {
        title
            .set_attribute(
                "style",
                "overflow: hidden; text-overflow: ellipsis; white-space: nowrap;",
            )
            .ok();
    }

    title
}

/// Paragraph props.
#[derive(Clone, Default)]
pub struct ParagraphProps {
    /// Paragraph text
    pub text: String,
    /// CSS class
    pub class: Option<String>,
    /// Muted style
    pub muted: bool,
    /// Code style
    pub code: bool,
    /// Strong (bold) style
    pub strong: bool,
    /// Delete (strikethrough) style
    pub delete: bool,
    /// Underline style
    pub underline: bool,
    /// Ellipsis for overflow
    pub ellipsis: bool,
    /// Number of rows before ellipsis
    pub rows: Option<u32>,
}

/// Build a Paragraph component.
pub fn paragraph(props: ParagraphProps) -> Element {
    let document = web_sys::window().unwrap().document().unwrap();

    let p: Element = document.create_element("p").unwrap();

    let mut classes = String::from("paragraph");
    if props.muted {
        classes.push_str(" paragraph-muted");
    }
    if let Some(class) = &props.class {
        classes.push_str(" ");
        classes.push_str(class);
    }
    p.set_class_name(&classes);

    if props.code {
        let code_el: Element = document.create_element("code").unwrap();
        code_el.set_text_content(Some(&props.text));
        p.append_child(&code_el).unwrap();
    } else if props.strong {
        let strong_el: Element = document.create_element("strong").unwrap();
        strong_el.set_text_content(Some(&props.text));
        p.append_child(&strong_el).unwrap();
    } else if props.delete {
        let del_el: Element = document.create_element("del").unwrap();
        del_el.set_text_content(Some(&props.text));
        p.append_child(&del_el).unwrap();
    } else if props.underline {
        let u_el: Element = document.create_element("u").unwrap();
        u_el.set_text_content(Some(&props.text));
        p.append_child(&u_el).unwrap();
    } else {
        p.set_text_content(Some(&props.text));
    }

    if props.ellipsis {
        if let Some(rows) = props.rows {
            p.set_attribute("style", &format!(
                "display: -webkit-box; -webkit-line-clamp: {}; -webkit-box-orient: vertical; overflow: hidden;",
                rows
            )).ok();
        } else {
            p.set_attribute(
                "style",
                "overflow: hidden; text-overflow: ellipsis; white-space: nowrap;",
            )
            .ok();
        }
    }

    p
}

/// Link props.
#[derive(Clone, Default)]
pub struct LinkProps {
    /// Link text
    pub text: String,
    /// Href URL
    pub href: String,
    /// CSS class
    pub class: Option<String>,
    /// Open in new tab
    pub target_blank: bool,
    /// External link icon
    pub external: bool,
}

/// Build a Link component.
pub fn link(props: LinkProps) -> Element {
    let document = web_sys::window().unwrap().document().unwrap();

    let a: Element = document.create_element("a").unwrap();
    a.set_class_name("link");
    a.set_attribute("href", &props.href).ok();

    if let Some(class) = &props.class {
        a.set_class_name(&format!("link {}", class));
    }

    if props.target_blank {
        a.set_attribute("target", "_blank").ok();
        a.set_attribute("rel", "noopener noreferrer").ok();
    }

    a.set_text_content(Some(&props.text));

    if props.external {
        let icon: Element = document.create_element("span").unwrap();
        icon.set_class_name("link-external-icon");
        icon.set_inner_html("&#8599;"); // ↗ symbol
        a.append_child(&icon).unwrap();
    }

    a
}

/// Text props.
#[derive(Clone, Default)]
pub struct TextProps {
    /// Text content
    pub text: String,
    /// CSS class
    pub class: Option<String>,
    /// Muted style
    pub muted: bool,
    /// Code style
    pub code: bool,
    /// Strong (bold) style
    pub strong: bool,
    /// Delete (strikethrough) style
    pub delete: bool,
    /// Underline style
    pub underline: bool,
    /// Mark (highlight) style
    pub mark: bool,
    /// Copyable text
    pub copyable: bool,
}

/// Build a Text component.
pub fn text(props: TextProps) -> Element {
    let document = web_sys::window().unwrap().document().unwrap();

    let span: Element = document.create_element("span").unwrap();

    let mut classes = String::from("text");
    if props.muted {
        classes.push_str(" text-muted");
    }
    if let Some(class) = &props.class {
        classes.push_str(" ");
        classes.push_str(class);
    }
    span.set_class_name(&classes);

    if props.code {
        let code_el: Element = document.create_element("code").unwrap();
        code_el.set_text_content(Some(&props.text));
        span.append_child(&code_el).unwrap();
    } else if props.strong {
        let strong_el: Element = document.create_element("strong").unwrap();
        strong_el.set_text_content(Some(&props.text));
        span.append_child(&strong_el).unwrap();
    } else if props.delete {
        let del_el: Element = document.create_element("del").unwrap();
        del_el.set_text_content(Some(&props.text));
        span.append_child(&del_el).unwrap();
    } else if props.underline {
        let u_el: Element = document.create_element("u").unwrap();
        u_el.set_text_content(Some(&props.text));
        span.append_child(&u_el).unwrap();
    } else if props.mark {
        let mark_el: Element = document.create_element("mark").unwrap();
        mark_el.set_text_content(Some(&props.text));
        span.append_child(&mark_el).unwrap();
    } else {
        span.set_text_content(Some(&props.text));
    }

    span
}
