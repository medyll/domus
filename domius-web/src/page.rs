//! `DomiusPage` trait — extends `DomiusComponent` with routing metadata.

use crate::component::DomiusComponent;

/// Extension of `DomiusComponent` for top-level page components.
///
/// Pages have a URL route, a browser title, and an async `on_load` hook
/// for data fetching or side-effects that should run on navigation.
///
/// # Example
///
/// ```ignore
/// struct HomePage;
///
/// impl DomiusComponent for HomePage {
///     type Props = ();
///     type State = ();
///     fn setup(_: ()) -> () { () }
///     fn render(_: &()) -> DomusNode { domius! { div { "Home" } } }
/// }
///
/// impl DomiusPage for HomePage {
///     fn route() -> &'static str { "/" }
///     fn title(_: &()) -> String { "Home".to_string() }
/// }
/// ```
pub trait DomiusPage: DomiusComponent {
    /// The URL path this page handles, e.g. `"/"` or `"/user/:id"`.
    fn route() -> &'static str;

    /// Browser tab title when this page is active.
    fn title(state: &Self::State) -> String;
}
