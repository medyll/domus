//! Timeline component - Chronological event list.

use web_sys::Element;

/// A single timeline event.
#[derive(Clone)]
pub struct TimelineEvent {
    pub title: String,
    pub description: Option<String>,
    pub timestamp: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
}

/// Timeline orientation.
#[derive(Clone, PartialEq)]
pub enum TimelineOrientation {
    Vertical,
    Horizontal,
}

impl Default for TimelineOrientation {
    fn default() -> Self {
        Self::Vertical
    }
}

/// Props for the Timeline component.
#[derive(Clone)]
pub struct TimelineProps {
    pub events: Vec<TimelineEvent>,
    pub orientation: TimelineOrientation,
    pub alternate: bool,
    pub class: Option<String>,
}

impl Default for TimelineProps {
    fn default() -> Self {
        Self {
            events: Vec::new(),
            orientation: TimelineOrientation::default(),
            alternate: false,
            class: None,
        }
    }
}

/// Timeline component.
pub struct Timeline;

impl Timeline {
    /// Create a timeline element.
    pub fn create(props: TimelineProps) -> Element {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");
        let timeline = document.create_element("ol").expect("create timeline");
        let orientation = match props.orientation {
            TimelineOrientation::Vertical => "vertical",
            TimelineOrientation::Horizontal => "horizontal",
        };
        let mut classes = vec!["domius-timeline", orientation];
        if props.alternate {
            classes.push("domius-timeline-alternate");
        }
        if let Some(class) = props.class.as_deref() {
            classes.push(class);
        }
        timeline.set_class_name(&classes.join(" "));
        timeline
            .set_attribute("data-orientation", orientation)
            .expect("set timeline orientation");

        for event in props.events {
            let item = document
                .create_element("li")
                .expect("create timeline event");
            item.set_class_name("domius-timeline-item");

            let marker = document
                .create_element("span")
                .expect("create timeline marker");
            marker.set_class_name("domius-timeline-marker");
            marker
                .set_attribute("aria-hidden", "true")
                .expect("hide timeline marker");
            if let Some(color) = event.color {
                marker
                    .set_attribute("data-color", &color)
                    .expect("set marker color token");
            }
            marker.set_text_content(event.icon.as_deref());
            item.append_child(&marker).expect("append timeline marker");

            let content = document
                .create_element("article")
                .expect("create timeline content");
            content.set_class_name("domius-timeline-content");
            let title = document
                .create_element("h3")
                .expect("create timeline title");
            title.set_text_content(Some(&event.title));
            content.append_child(&title).expect("append timeline title");

            if let Some(timestamp) = event.timestamp {
                let time = document
                    .create_element("time")
                    .expect("create timeline timestamp");
                time.set_class_name("domius-timeline-time");
                time.set_text_content(Some(&timestamp));
                content.append_child(&time).expect("append timeline time");
            }
            if let Some(description) = event.description {
                let paragraph = document
                    .create_element("p")
                    .expect("create timeline description");
                paragraph.set_text_content(Some(&description));
                content
                    .append_child(&paragraph)
                    .expect("append timeline description");
            }
            item.append_child(&content)
                .expect("append timeline content");
            timeline.append_child(&item).expect("append timeline event");
        }

        timeline
    }
}
