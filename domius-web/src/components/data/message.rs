//! Chat/Messages component for Domius.
//!
//! Chat bubble display for conversations.

use web_sys::Element;

/// Message position.
#[derive(Clone, Copy, Default, PartialEq)]
pub enum MessagePosition {
    #[default]
    Left,
    Right,
}

/// Message props.
#[derive(Clone, Default)]
pub struct MessageProps {
    /// Message text
    pub text: String,
    /// Sender name
    pub sender: Option<String>,
    /// Avatar URL
    pub avatar: Option<String>,
    /// Message position
    pub position: MessagePosition,
    /// CSS class
    pub class: Option<String>,
    /// Timestamp
    pub timestamp: Option<String>,
    /// Read status
    pub read: bool,
}

/// Build a Message component.
///
/// # Example
///
/// ```ignore
/// use domius_web::components::message::{message, MessageProps, MessagePosition};
///
/// let message_node = message(MessageProps {
///     text: "Hello!".to_string(),
///     sender: Some("John".to_string()),
///     position: MessagePosition::Left,
///     ..Default::default()
/// });
/// ```
pub fn message(props: MessageProps) -> Element {
    let document = web_sys::window().unwrap().document().unwrap();

    let container: Element = document.create_element("div").unwrap();

    let mut classes = String::from("message");
    if props.position == MessagePosition::Right {
        classes.push_str(" message-right");
    } else {
        classes.push_str(" message-left");
    }
    if let Some(class) = &props.class {
        classes.push_str(" ");
        classes.push_str(class);
    }
    container.set_class_name(&classes);

    // Avatar (only for left messages)
    if props.position == MessagePosition::Left {
        let avatar_container: Element = document.create_element("div").unwrap();
        avatar_container.set_class_name("message-avatar");

        if let Some(avatar) = &props.avatar {
            let avatar_img: Element = document.create_element("img").unwrap();
            avatar_img.set_attribute("src", avatar).ok();
            avatar_img.set_attribute("alt", "avatar").ok();
            avatar_container.append_child(&avatar_img).unwrap();
        } else if let Some(sender) = &props.sender {
            // Fallback to initials
            let initials: Element = document.create_element("span").unwrap();
            initials.set_class_name("message-avatar-initials");
            let initial = sender
                .chars()
                .next()
                .unwrap_or('?')
                .to_uppercase()
                .to_string();
            initials.set_text_content(Some(&initial));
            avatar_container.append_child(&initials).unwrap();
        }

        container.append_child(&avatar_container).unwrap();
    }

    // Message content
    let content: Element = document.create_element("div").unwrap();
    content.set_class_name("message-content");

    // Sender name (optional)
    if let Some(sender) = &props.sender {
        let sender_el: Element = document.create_element("div").unwrap();
        sender_el.set_class_name("message-sender");
        sender_el.set_text_content(Some(sender));
        content.append_child(&sender_el).unwrap();
    }

    // Message bubble
    let bubble: Element = document.create_element("div").unwrap();
    bubble.set_class_name("message-bubble");
    bubble.set_text_content(Some(&props.text));
    content.append_child(&bubble).unwrap();

    // Timestamp and read status
    let meta: Element = document.create_element("div").unwrap();
    meta.set_class_name("message-meta");

    if let Some(timestamp) = &props.timestamp {
        let time_el: Element = document.create_element("span").unwrap();
        time_el.set_class_name("message-time");
        time_el.set_text_content(Some(timestamp));
        meta.append_child(&time_el).unwrap();
    }

    if props.read && props.position == MessagePosition::Right {
        let read_el: Element = document.create_element("span").unwrap();
        read_el.set_class_name("message-read");
        read_el.set_inner_html("&#10003;&#10003;"); // ✓✓ double check
        meta.append_child(&read_el).unwrap();
    }

    content.append_child(&meta).unwrap();
    container.append_child(&content).unwrap();

    container
}

/// Chat container props.
#[derive(Clone, Default)]
pub struct ChatProps {
    /// CSS class
    pub class: Option<String>,
    /// Chat title
    pub title: Option<String>,
}

/// Build a Chat container component.
pub fn chat(props: ChatProps) -> Element {
    let document = web_sys::window().unwrap().document().unwrap();

    let container: Element = document.create_element("div").unwrap();

    let mut classes = String::from("chat");
    if let Some(class) = &props.class {
        classes.push_str(" ");
        classes.push_str(class);
    }
    container.set_class_name(&classes);

    // Header
    if let Some(title) = &props.title {
        let header: Element = document.create_element("div").unwrap();
        header.set_class_name("chat-header");

        let title_el: Element = document.create_element("h3").unwrap();
        title_el.set_text_content(Some(title));
        header.append_child(&title_el).unwrap();

        container.append_child(&header).unwrap();
    }

    // Messages container
    let messages: Element = document.create_element("div").unwrap();
    messages.set_class_name("chat-messages");
    container.append_child(&messages).unwrap();

    // Input area
    let input_area: Element = document.create_element("div").unwrap();
    input_area.set_class_name("chat-input");

    let input: Element = document.create_element("input").unwrap();
    input.set_attribute("type", "text").ok();
    input.set_class_name("chat-input-field");
    input.set_attribute("placeholder", "Type a message...").ok();

    let send_btn: Element = document.create_element("button").unwrap();
    send_btn.set_class_name("chat-send-btn");
    send_btn.set_inner_html("&#10148;"); // ➤ symbol

    input_area.append_child(&input).unwrap();
    input_area.append_child(&send_btn).unwrap();

    container.append_child(&input_area).unwrap();

    container
}
