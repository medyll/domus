//! Component system for Domius desktop applications.
//!
//! Components provide state and configuration for Domius-managed windows.

use domius_core::{create_scope, dispose_scope, ScopeId};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Trait for desktop components.
///
/// Desktop components are mounted into windows owned by the Domius runtime.
pub trait DomiusDesktopComponent {
    /// Props type for this component.
    type Props: Clone + Send + 'static;
    /// State type for this component.
    type State: 'static;

    /// Setup the component state from props.
    fn setup(props: Self::Props) -> Self::State;

    /// Get the window title.
    fn title(_state: &Self::State) -> String {
        "Domus App".into()
    }

    /// Get the window label (for internal tracking).
    fn label() -> &'static str {
        "domus_window"
    }

    /// Get the initial window size (width, height).
    fn window_size() -> (u32, u32) {
        (800, 600)
    }

    /// Get the URL to load in the webview.
    fn url() -> &'static str {
        "index.html"
    }
}

/// Associates a component scope with a Domius window.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentScope {
    pub scope_id: ScopeId,
    pub window_label: String,
}

/// Build a window configuration for a desktop component.
///
/// Returns the configuration needed to create a window with the component.
pub fn build_window_config<C: DomiusDesktopComponent>(
    props: C::Props,
) -> (ScopeId, String, String, (u32, u32)) {
    let state = C::setup(props);
    let scope = create_scope(None);

    let window_label = format!("{}_{}", C::label(), scope.value());
    let title = C::title(&state);
    let (width, height) = C::window_size();

    (scope, window_label, title, (width, height))
}

/// Cleanup a component scope when window closes.
pub fn cleanup_component_scope(scope_id: ScopeId) {
    dispose_scope(scope_id);
}

/// Return the content URL declared by a desktop component.
///
/// A Domius window host uses this together with [`build_window_config`] when
/// mounting the component.
/// ```ignore
/// let (scope, label, title, (w, h)) = build_window_config::<MyComponent>(props);
/// let url = get_component_url::<MyComponent>();
/// desktop_runtime.open_window(scope, label, title, (w, h), url)?;
/// ```
pub fn get_component_url<C: DomiusDesktopComponent>() -> &'static str {
    C::url()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestComponent;

    #[derive(Clone)]
    struct TestProps {
        value: i32,
    }

    struct TestState {
        value: i32,
    }

    impl DomiusDesktopComponent for TestComponent {
        type Props = TestProps;
        type State = TestState;

        fn setup(props: Self::Props) -> Self::State {
            TestState { value: props.value }
        }

        fn title(state: &Self::State) -> String {
            format!("Test: {}", state.value)
        }

        fn url() -> &'static str {
            "test.html"
        }
    }

    #[test]
    fn component_trait_works() {
        let state = TestComponent::setup(TestProps { value: 42 });
        assert_eq!(state.value, 42);
        assert_eq!(TestComponent::title(&state), "Test: 42");
    }

    #[test]
    fn window_size_default() {
        assert_eq!(TestComponent::window_size(), (800, 600));
    }

    #[test]
    fn build_config_returns_values() {
        let (scope, label, title, size) =
            build_window_config::<TestComponent>(TestProps { value: 10 });
        assert_eq!(title, "Test: 10");
        assert!(label.starts_with("domus_window_"));
        assert_eq!(size, (800, 600));
        cleanup_component_scope(scope);
    }
}
