//! Comment component for Domius.
//!
//! Comment display with avatar, author, content, and actions.

use web_sys::Element;

/// Comment action.
#[derive(Clone)]
pub struct CommentAction {
    pub text: String,
    pub on_click: Option<String>, // Callback identifier
}

/// Comment props.
#[derive(Clone, Default)]
pub struct CommentProps {
    /// Comment author name
    pub author: Option<String>,
    /// Author avatar URL
    pub avatar: Option<String>,
    /// Comment content
    pub content: String,
    /// Comment datetime
    pub datetime: Option<String>,
    /// CSS class
    pub class: Option<String>,
    /// Actions (reply, edit, delete, etc.)
    pub actions: Option<Vec<CommentAction>>,
    /// Nested replies
    pub children: Option<Vec<CommentProps>>,
    /// Upvote count
    pub upvotes: Option<u32>,
    /// Downvote count
    pub downvotes: Option<u32>,
}

/// Build a Comment component.
///
/// # Example
///
/// ```ignore
/// use domius_web::components::comment::{comment, CommentProps};
///
/// let comment_node = comment(CommentProps {
///     author: Some("John Doe".to_string()),
///     avatar: Some("avatar.jpg".to_string()),
///     content: "This is a comment".to_string(),
///     datetime: Some("2 hours ago".to_string()),
///     ..Default::default()
/// });
/// ```
pub fn comment(props: CommentProps) -> Element {
    let document = web_sys::window().unwrap().document().unwrap();
    
    let container: Element = document.create_element("div").unwrap();
    
    let mut classes = String::from("comment");
    if let Some(class) = &props.class {
        classes.push_str(" ");
        classes.push_str(class);
    }
    container.set_class_name(&classes);

    // Avatar
    let avatar_container: Element = document.create_element("div").unwrap();
    avatar_container.set_class_name("comment-avatar");
    
    if let Some(avatar) = &props.avatar {
        let avatar_img: Element = document.create_element("img").unwrap();
        avatar_img.set_attribute("src", avatar).ok();
        avatar_img.set_attribute("alt", "avatar").ok();
        avatar_container.append_child(&avatar_img).unwrap();
    } else if let Some(author) = &props.author {
        // Fallback to initials
        let initials: Element = document.create_element("span").unwrap();
        initials.set_class_name("comment-avatar-initials");
        let initial = author.chars().next().unwrap_or('?').to_uppercase().to_string();
        initials.set_text_content(Some(&initial));
        avatar_container.append_child(&initials).unwrap();
    }
    
    container.append_child(&avatar_container).unwrap();

    // Content
    let content_container: Element = document.create_element("div").unwrap();
    content_container.set_class_name("comment-content");

    // Author and datetime header
    let header: Element = document.create_element("div").unwrap();
    header.set_class_name("comment-header");
    
    if let Some(author) = &props.author {
        let author_el: Element = document.create_element("span").unwrap();
        author_el.set_class_name("comment-author");
        author_el.set_text_content(Some(author));
        header.append_child(&author_el).unwrap();
    }
    
    if let Some(datetime) = &props.datetime {
        let datetime_el: Element = document.create_element("span").unwrap();
        datetime_el.set_class_name("comment-datetime");
        datetime_el.set_text_content(Some(datetime));
        header.append_child(&datetime_el).unwrap();
    }
    
    content_container.append_child(&header).unwrap();

    // Comment text
    let text_el: Element = document.create_element("div").unwrap();
    text_el.set_class_name("comment-text");
    text_el.set_text_content(Some(&props.content));
    content_container.append_child(&text_el).unwrap();

    // Actions
    if let Some(actions) = &props.actions {
        let actions_el: Element = document.create_element("div").unwrap();
        actions_el.set_class_name("comment-actions");
        
        for (i, action) in actions.iter().enumerate() {
            let action_el: Element = document.create_element("span").unwrap();
            action_el.set_class_name("comment-action");
            action_el.set_text_content(Some(&action.text));
            actions_el.append_child(&action_el).unwrap();
            
            if i < actions.len() - 1 {
                let separator: Element = document.create_element("span").unwrap();
                separator.set_class_name("comment-action-separator");
                separator.set_text_content(Some("·"));
                actions_el.append_child(&separator).unwrap();
            }
        }
        
        content_container.append_child(&actions_el).unwrap();
    }

    // Votes
    if props.upvotes.is_some() || props.downvotes.is_some() {
        let votes_el: Element = document.create_element("div").unwrap();
        votes_el.set_class_name("comment-votes");
        
        if let Some(upvotes) = props.upvotes {
            let upvote_el: Element = document.create_element("span").unwrap();
            upvote_el.set_class_name("comment-upvotes");
            upvote_el.set_inner_html("&#9650;"); // ▲ symbol
            upvote_el.set_attribute("data-count", &upvotes.to_string()).ok();
            votes_el.append_child(&upvote_el).unwrap();
        }
        
        if let Some(downvotes) = props.downvotes {
            let downvote_el: Element = document.create_element("span").unwrap();
            downvote_el.set_class_name("comment-downvotes");
            downvote_el.set_inner_html("&#9660;"); // ▼ symbol
            downvote_el.set_attribute("data-count", &downvotes.to_string()).ok();
            votes_el.append_child(&downvote_el).unwrap();
        }
        
        content_container.append_child(&votes_el).unwrap();
    }

    container.append_child(&content_container).unwrap();

    // Nested replies
    if let Some(children) = &props.children {
        let replies: Element = document.create_element("div").unwrap();
        replies.set_class_name("comment-replies");
        
        for child in children {
            let child_el = comment(child.clone());
            replies.append_child(&child_el).unwrap();
        }
        
        container.append_child(&replies).unwrap();
    }

    container
}

/// Comment list props.
#[derive(Clone, Default)]
pub struct CommentListProps {
    /// List of comments
    pub comments: Vec<CommentProps>,
    /// CSS class
    pub class: Option<String>,
    /// Header title
    pub header: Option<String>,
    /// Show comment count
    pub show_count: bool,
}

/// Build a CommentList component.
pub fn comment_list(props: CommentListProps) -> Element {
    let document = web_sys::window().unwrap().document().unwrap();
    
    let container: Element = document.create_element("div").unwrap();
    
    let mut classes = String::from("comment-list");
    if let Some(class) = &props.class {
        classes.push_str(" ");
        classes.push_str(class);
    }
    container.set_class_name(&classes);

    // Header
    if props.header.is_some() || props.show_count {
        let header: Element = document.create_element("div").unwrap();
        header.set_class_name("comment-list-header");
        
        if let Some(header_text) = &props.header {
            let title: Element = document.create_element("h3").unwrap();
            title.set_text_content(Some(header_text));
            header.append_child(&title).unwrap();
        }
        
        if props.show_count {
            let count: Element = document.create_element("span").unwrap();
            count.set_class_name("comment-count");
            count.set_text_content(Some(&format!("{} Comments", props.comments.len())));
            header.append_child(&count).unwrap();
        }
        
        container.append_child(&header).unwrap();
    }

    // Comments
    let comments_container: Element = document.create_element("div").unwrap();
    comments_container.set_class_name("comment-list-items");
    
    for comment_props in &props.comments {
        let comment_el = comment(comment_props.clone());
        comments_container.append_child(&comment_el).unwrap();
    }
    
    container.append_child(&comments_container).unwrap();

    container
}
