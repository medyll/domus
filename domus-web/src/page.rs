//! `DomusPage` trait — extends `DomusComponent` with routing metadata.

use crate::component::DomusComponent;

/// Extension of `DomusComponent` for top-level page components.
///
/// Pages have a URL route, a browser title, and an async `on_load` hook
/// for data fetching or side-effects that should run on navigation.
///
/// # Example
///
/// ```ignore
/// struct HomePage;
///
/// impl DomusComponent for HomePage {
///     type Props = ();
///     type State = ();
///     fn setup(_: ()) -> () { () }
///     fn render(_: &()) -> DomusNode { domus! { div { "Home" } } }
/// }
///
/// impl DomusPage for HomePage {
///     fn route() -> &'static str { "/" }
///     fn title(_: &()) -> String { "Home".to_string() }
/// }
/// ```
pub trait DomusPage: DomusComponent {
    /// The URL path this page handles, e.g. `"/"` or `"/user/:id"`.
    fn route() -> &'static str;

    /// Browser tab title when this page is active.
    fn title(state: &Self::State) -> String;
}
