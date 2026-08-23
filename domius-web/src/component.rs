//! `DomiusComponent` trait and supporting types for the Domius component model.

/// The rendered output of a component — a live DOM element.
pub type DomiusNode = web_sys::Element;

/// Core trait every Domius component must implement.
///
/// # Type Parameters
///
/// - `Props` — immutable input data passed when mounting
/// - `State` — reactive state created from props during setup
///
/// # Lifecycle
///
/// 1. `setup(props)` — called once on mount; creates signals and state
/// 2. `render(state)` — called immediately after setup; returns a DOM subtree
///
/// When signals inside `State` change, the generated effects (wired by the
/// `domus!` macro) update the DOM automatically — `render` itself is **not**
/// called again.
///
/// # Example
///
/// ```ignore
/// struct Counter;
///
/// struct CounterProps {
///     initial: i32,
/// }
///
/// struct CounterState {
///     count: Signal<i32>,
/// }
///
/// impl DomiusComponent for Counter {
///     type Props = CounterProps;
///     type State = CounterState;
///
///     fn setup(props: CounterProps) -> CounterState {
///         CounterState { count: signal(props.initial) }
///     }
///
///     fn render(state: &CounterState) -> DomiusNode {
///         let count = state.count.clone();
///         domius! { div { "Count: " {count} } }
///     }
/// }
/// ```
pub trait DomiusComponent {
    /// Immutable configuration passed from parent to this component.
    type Props;

    /// Reactive state owned by this component instance.
    type State;

    /// Called once when the component is mounted.
    /// Create all reactive signals here; return the state bundle.
    fn setup(props: Self::Props) -> Self::State;

    /// Produce the DOM subtree for this component.
    /// Called once after `setup`; DOM is kept live via signal effects.
    fn render(state: &Self::State) -> DomiusNode;

    /// Mount this component as a child of `parent`.
    /// Default implementation: calls `setup` + `render` + `append_child`.
    fn mount(props: Self::Props, parent: &web_sys::Element) -> Self::State {
        let state = Self::setup(props);
        let node = Self::render(&state);
        parent
            .append_child(&node)
            .expect("DomiusComponent::mount — failed to append node");
        state
    }
}

/// Mount a component by type, returning its state.
///
/// ```ignore
/// let state = mount_component::<Counter>(CounterProps { initial: 0 }, &body);
/// ```
pub fn mount_component<C: DomiusComponent>(props: C::Props, parent: &web_sys::Element) -> C::State {
    C::mount(props, parent)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---------------------------------------------------------------------------
    // Minimal mock so DomiusComponent trait can be tested without a real DOM
    // ---------------------------------------------------------------------------

    /// Stand-in for `DomiusNode` in tests (not web-sys dependent in type-check)
    struct MockNode;

    struct NoProps;
    struct NoState;

    /// Minimal component with unit props/state
    struct NullComponent;

    // We can't impl DomiusComponent with MockNode since DomiusNode = web_sys::Element.
    // Instead, test the trait pattern using a standalone struct.
    //
    // These tests verify that the trait's associated types and method signatures
    // are structurally correct by checking a concrete implementation compiles.
    //
    // Full mount tests require WASM (`wasm-pack test`).

    // ---------------------------------------------------------------------------
    // Props / State type tests
    // ---------------------------------------------------------------------------

    /// A concrete component that stores state.
    struct StatefulComponent;

    struct ButtonProps {
        label: String,
        disabled: bool,
    }

    struct ButtonState {
        label: String,
        disabled: bool,
        click_count: u32,
    }

    /// Verify the trait can be implemented with non-trivial Props/State.
    /// We use a wrapper trait with a mock node to avoid requiring WASM.
    trait TestComponent {
        type Props;
        type State;
        fn setup(props: Self::Props) -> Self::State;
    }

    impl TestComponent for StatefulComponent {
        type Props = ButtonProps;
        type State = ButtonState;

        fn setup(props: ButtonProps) -> ButtonState {
            ButtonState {
                label: props.label,
                disabled: props.disabled,
                click_count: 0,
            }
        }
    }

    #[test]
    fn test_component_setup_builds_state_from_props() {
        let props = ButtonProps {
            label: "Click me".to_string(),
            disabled: false,
        };
        let state = StatefulComponent::setup(props);
        assert_eq!(state.label, "Click me");
        assert!(!state.disabled);
        assert_eq!(state.click_count, 0);
    }

    #[test]
    fn test_component_state_is_independent_per_instance() {
        let state_a = StatefulComponent::setup(ButtonProps {
            label: "A".to_string(),
            disabled: false,
        });
        let state_b = StatefulComponent::setup(ButtonProps {
            label: "B".to_string(),
            disabled: true,
        });
        assert_ne!(state_a.label, state_b.label);
        assert_ne!(state_a.disabled, state_b.disabled);
    }

    #[test]
    fn test_unit_props_component() {
        struct UnitComp;
        impl TestComponent for UnitComp {
            type Props = ();
            type State = u32;
            fn setup(_: ()) -> u32 {
                42
            }
        }
        let state = UnitComp::setup(());
        assert_eq!(state, 42);
    }

    #[test]
    fn test_component_with_computed_initial_state() {
        struct ComputedComp;
        struct ComputedProps {
            values: Vec<i32>,
        }
        struct ComputedState {
            sum: i32,
            count: usize,
        }
        impl TestComponent for ComputedComp {
            type Props = ComputedProps;
            type State = ComputedState;
            fn setup(props: ComputedProps) -> ComputedState {
                ComputedState {
                    sum: props.values.iter().sum(),
                    count: props.values.len(),
                }
            }
        }
        let state = ComputedComp::setup(ComputedProps {
            values: vec![1, 2, 3, 4],
        });
        assert_eq!(state.sum, 10);
        assert_eq!(state.count, 4);
    }

    #[test]
    fn test_domus_component_trait_associated_types_accessible() {
        // Compile-time check: verify Props/State associated types are accessible.
        fn _assert_props<C: DomiusComponent>(_: &C::Props) {}
        fn _assert_state<C: DomiusComponent>(_: &C::State) {}
        // If this function compiles, the associated types are correctly declared.
        // Actual invocation not needed — type-checking is sufficient.
        let _ = std::any::type_name::<StatefulComponent>();
    }
}
