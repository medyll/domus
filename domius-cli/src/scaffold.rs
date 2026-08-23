//! Code and project scaffold generation for Domius CLI.
//!
//! All functions return `ScaffoldFile` lists (path + content strings) and
//! are fully testable without touching the file system.

/// A single file to be generated: relative path + UTF-8 content.
#[derive(Debug, Clone, PartialEq)]
pub struct ScaffoldFile {
    /// Relative path from the project root (e.g. `"src/main.rs"`).
    pub path: String,
    /// File content.
    pub content: String,
}

impl ScaffoldFile {
    fn new(path: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            content: content.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// `domus new project <name>`
// ---------------------------------------------------------------------------

/// Generate all files for a new Domius project named `name`.
pub fn new_project(name: &str) -> Vec<ScaffoldFile> {
    let snake = to_snake_case(name);
    vec![
        ScaffoldFile::new(
            format!("{}/Cargo.toml", name),
            format!(
                r#"[package]
name = "{snake}"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
domius-web = {{ path = "../../domius-web" }}
domius-core = {{ path = "../../domius-core" }}
wasm-bindgen = "0.2"
web-sys = {{ version = "0.3", features = ["Window", "Document", "Element", "Node"] }}
"#,
                snake = snake
            ),
        ),
        ScaffoldFile::new(
            format!("{}/src/lib.rs", name),
            format!(
                r#"//! {name} — a Domius WASM application.

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {{
    domius_web::init();
    // Mount your root component here
}}
"#,
                name = name
            ),
        ),
        ScaffoldFile::new(
            format!("{}/src/routes.rs", name),
            r#"//! Route registration.
//!
//! Call `register_routes` from your entry point.
use domius_web::router::Router;

pub fn register_routes() -> Router<fn()> {
    let mut router = Router::new();
    // router.register("/", home_page);
    router
}
"#
            .to_string(),
        ),
        ScaffoldFile::new(
            format!("{}/index.html", name),
            format!(
                r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>{name}</title>
</head>
<body>
  <div id="app"></div>
  <script type="module">
    import init from "./pkg/{snake}.js";
    init();
  </script>
</body>
</html>
"#,
                name = name,
                snake = snake
            ),
        ),
        ScaffoldFile::new(
            format!("{}/.gitignore", name),
            "/target\n/pkg\n".to_string(),
        ),
    ]
}

// ---------------------------------------------------------------------------
// `domus add component <name>`
// ---------------------------------------------------------------------------

/// Generate files for a new reusable component named `name`.
pub fn new_component(name: &str) -> Vec<ScaffoldFile> {
    let snake = to_snake_case(name);
    let pascal = to_pascal_case(name);

    vec![
        ScaffoldFile::new(
            format!("src/components/{}/mod.rs", snake),
            format!(
                r#"//! `{pascal}` component.
pub mod view;
pub use view::{pascal};
"#,
                pascal = pascal
            ),
        ),
        ScaffoldFile::new(
            format!("src/components/{}/view.rs", snake),
            format!(
                r#"//! View for `{pascal}`.
use domius_web::component::{{DomiusComponent, DomiusNode}};

pub struct {pascal};

#[derive(Clone)]
pub struct {pascal}Props {{
    // Add props here
}}

pub struct {pascal}State {{
    // Add reactive state here
}}

impl DomiusComponent for {pascal} {{
    type Props = {pascal}Props;
    type State = {pascal}State;

    fn setup(props: {pascal}Props) -> {pascal}State {{
        let _ = props;
        {pascal}State {{}}
    }}

    fn render(state: &{pascal}State) -> DomiusNode {{
        let _ = state;
        todo!("implement {pascal}::render")
    }}
}}
"#,
                pascal = pascal
            ),
        ),
        ScaffoldFile::new(
            format!("src/components/{}/style.css", snake),
            format!("/* Styles for {} component */\n.{} {{\n}}\n", pascal, snake),
        ),
    ]
}

// ---------------------------------------------------------------------------
// `domus add page <name>`
// ---------------------------------------------------------------------------

/// Generate files for a new page named `name`.
pub fn new_page(name: &str) -> Vec<ScaffoldFile> {
    let snake = to_snake_case(name);
    let pascal = to_pascal_case(name);
    let route = format!("/{}", snake);

    vec![
        ScaffoldFile::new(
            format!("src/pages/{}/mod.rs", snake),
            format!(
                r#"//! `{pascal}Page`.
pub mod controller;
pub mod view;
pub use view::{pascal}Page;
"#,
                pascal = pascal
            ),
        ),
        ScaffoldFile::new(
            format!("src/pages/{}/controller.rs", snake),
            format!(
                r#"//! Controller (setup logic) for `{pascal}Page`.
use domius_core::signal::{{Signal, signal}};

pub struct {pascal}State {{
    // Add reactive signals here
}}

pub fn setup() -> {pascal}State {{
    {pascal}State {{}}
}}
"#,
                pascal = pascal
            ),
        ),
        ScaffoldFile::new(
            format!("src/pages/{}/view.rs", snake),
            format!(
                r#"//! View for `{pascal}Page`.
use domius_web::component::{{DomiusComponent, DomiusNode}};
use domius_web::page::DomiusPage;
use super::controller;

pub struct {pascal}Page;

impl DomiusComponent for {pascal}Page {{
    type Props = ();
    type State = controller::{pascal}State;

    fn setup(_: ()) -> controller::{pascal}State {{
        controller::setup()
    }}

    fn render(state: &controller::{pascal}State) -> DomiusNode {{
        let _ = state;
        todo!("implement {pascal}Page::render")
    }}
}}

impl DomiusPage for {pascal}Page {{
    fn route() -> &'static str {{ "{route}" }}
    fn title(_: &controller::{pascal}State) -> String {{ "{pascal}".to_string() }}
}}
"#,
                pascal = pascal,
                route = route
            ),
        ),
        ScaffoldFile::new(
            format!("src/pages/{}/style.css", snake),
            format!("/* Styles for {} page */\n", pascal),
        ),
    ]
}

// ---------------------------------------------------------------------------
// Naming helpers
// ---------------------------------------------------------------------------

/// `"MyComponent"` → `"my_component"`
pub fn to_snake_case(s: &str) -> String {
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() && i > 0 {
            out.push('_');
        }
        out.extend(ch.to_lowercase());
    }
    // Also handle already-snake and kebab input: replace `-` → `_`
    out.replace('-', "_")
}

/// `"my_component"` / `"my-component"` / `"myComponent"` → `"MyComponent"`
pub fn to_pascal_case(s: &str) -> String {
    s.split(['_', '-'])
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- Naming helpers ---

    #[test]
    fn test_to_snake_case_pascal() {
        assert_eq!(to_snake_case("MyComponent"), "my_component");
    }

    #[test]
    fn test_to_snake_case_already_snake() {
        assert_eq!(to_snake_case("my_component"), "my_component");
    }

    #[test]
    fn test_to_snake_case_kebab() {
        assert_eq!(to_snake_case("my-component"), "my_component");
    }

    #[test]
    fn test_to_pascal_case_from_snake() {
        assert_eq!(to_pascal_case("my_component"), "MyComponent");
    }

    #[test]
    fn test_to_pascal_case_from_kebab() {
        assert_eq!(to_pascal_case("my-component"), "MyComponent");
    }

    #[test]
    fn test_to_pascal_case_already_pascal() {
        assert_eq!(to_pascal_case("MyComponent"), "MyComponent");
    }

    #[test]
    fn test_to_pascal_case_single_word() {
        assert_eq!(to_pascal_case("button"), "Button");
    }

    // --- new_project ---

    #[test]
    fn test_new_project_generates_expected_files() {
        let files = new_project("my-app");
        let paths: Vec<&str> = files.iter().map(|f| f.path.as_str()).collect();
        assert!(paths.contains(&"my-app/Cargo.toml"));
        assert!(paths.contains(&"my-app/src/lib.rs"));
        assert!(paths.contains(&"my-app/src/routes.rs"));
        assert!(paths.contains(&"my-app/index.html"));
        assert!(paths.contains(&"my-app/.gitignore"));
    }

    #[test]
    fn test_new_project_cargo_toml_has_correct_name() {
        let files = new_project("todo-app");
        let cargo = files
            .iter()
            .find(|f| f.path.ends_with("Cargo.toml"))
            .unwrap();
        assert!(cargo.content.contains("name = \"todo_app\""));
    }

    #[test]
    fn test_new_project_index_html_has_app_div() {
        let files = new_project("hello");
        let html = files
            .iter()
            .find(|f| f.path.ends_with("index.html"))
            .unwrap();
        assert!(html.content.contains(r#"<div id="app">"#));
    }

    #[test]
    fn test_new_project_lib_rs_has_wasm_bindgen_start() {
        let files = new_project("hello");
        let lib = files.iter().find(|f| f.path.ends_with("lib.rs")).unwrap();
        assert!(lib.content.contains("wasm_bindgen(start)"));
    }

    // --- new_component ---

    #[test]
    fn test_new_component_generates_expected_files() {
        let files = new_component("Button");
        let paths: Vec<&str> = files.iter().map(|f| f.path.as_str()).collect();
        assert!(paths.contains(&"src/components/button/mod.rs"));
        assert!(paths.contains(&"src/components/button/view.rs"));
        assert!(paths.contains(&"src/components/button/style.css"));
    }

    #[test]
    fn test_new_component_view_has_impl_domus_component() {
        let files = new_component("NavBar");
        let view = files.iter().find(|f| f.path.ends_with("view.rs")).unwrap();
        assert!(view.content.contains("impl DomiusComponent for NavBar"));
    }

    #[test]
    fn test_new_component_props_and_state_named_correctly() {
        let files = new_component("SearchBox");
        let view = files.iter().find(|f| f.path.ends_with("view.rs")).unwrap();
        assert!(view.content.contains("SearchBoxProps"));
        assert!(view.content.contains("SearchBoxState"));
    }

    // --- new_page ---

    #[test]
    fn test_new_page_generates_expected_files() {
        let files = new_page("Dashboard");
        let paths: Vec<&str> = files.iter().map(|f| f.path.as_str()).collect();
        assert!(paths.contains(&"src/pages/dashboard/mod.rs"));
        assert!(paths.contains(&"src/pages/dashboard/controller.rs"));
        assert!(paths.contains(&"src/pages/dashboard/view.rs"));
        assert!(paths.contains(&"src/pages/dashboard/style.css"));
    }

    #[test]
    fn test_new_page_view_impl_domus_page() {
        let files = new_page("Home");
        let view = files.iter().find(|f| f.path.ends_with("view.rs")).unwrap();
        assert!(view.content.contains("impl DomiusPage for HomePage"));
    }

    #[test]
    fn test_new_page_route_is_snake_case_path() {
        let files = new_page("UserProfile");
        let view = files.iter().find(|f| f.path.ends_with("view.rs")).unwrap();
        assert!(view.content.contains("\"/user_profile\""));
    }

    #[test]
    fn test_new_page_controller_has_setup_fn() {
        let files = new_page("Settings");
        let ctrl = files
            .iter()
            .find(|f| f.path.ends_with("controller.rs"))
            .unwrap();
        assert!(ctrl.content.contains("pub fn setup()"));
    }

    #[test]
    fn test_scaffold_file_equality() {
        let f1 = ScaffoldFile::new("path/to/file.rs", "content");
        let f2 = ScaffoldFile::new("path/to/file.rs", "content");
        assert_eq!(f1, f2);
    }
}
