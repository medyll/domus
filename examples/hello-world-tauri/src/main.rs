//! Hello World example for Domus Desktop (Tauri)
//!
//! This example demonstrates a simple counter component running in a Tauri window.

use domius_core::signal::{signal, Signal};
use domius_core::effect::create_effect;
use domius_desktop::{
    DomiusDesktopComponent, build_window_config, cleanup_component_scope,
};

// ============================================================================
// Counter Component
// ============================================================================

pub struct CounterComponent;

#[derive(Clone)]
pub struct CounterProps {
    pub initial: i32,
}

pub struct CounterState {
    pub count: Signal<i32>,
    pub set_count: Signal<i32>,
}

impl DomiusDesktopComponent for CounterComponent {
    type Props = CounterProps;
    type State = CounterState;

    fn setup(props: CounterProps) -> Self::State {
        let (count, set_count) = signal(props.initial);
        
        // Log count changes (for demonstration)
        let count_clone = count.clone();
        create_effect(move || {
            println!("Counter changed: {}", count_clone.get());
        });
        
        CounterState { count, set_count }
    }

    fn title(_state: &Self::State) -> String {
        "Domus Counter".into()
    }

    fn label() -> &'static str {
        "counter"
    }

    fn window_size() -> (u32, u32) {
        (400, 300)
    }

    fn url() -> &'static str {
        "index.html"
    }
}

// ============================================================================
// Tauri Application
// ============================================================================

fn main() {
    println!("Starting Domus Desktop Hello World...");

    tauri::Builder::default()
        .setup(|app| {
            // Create the counter component
            let props = CounterProps { initial: 0 };
            let (scope, label, title, (width, height)) = 
                build_window_config::<CounterComponent>(props);

            println!("Creating window: {} - {}x{}", label, width, height);

            // Create the main window
            let window = tauri::window::WindowBuilder::new(app, label.clone())
                .title(title)
                .inner_size(width as f64, height as f64)
                .resizable(true)
                .build()?;

            // Set up window close handler
            let scope_clone = scope;
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { .. } = event {
                    println!("Window closing, disposing scope...");
                    cleanup_component_scope(scope_clone);
                }
            });

            // Load the HTML file from assets
            window.load_url(tauri::Url::parse("app://local/index.html")?)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
