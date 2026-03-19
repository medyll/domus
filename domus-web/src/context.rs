//! Type-safe context API for sharing state without prop drilling.
//!
//! Contexts are stored in a `thread_local` registry keyed by [`TypeId`].
//! Values are cloned on retrieval, so `T` must implement [`Clone`].
//!
//! # Example
//!
//! ```
//! use domus_web::context::{provide_context, use_context};
//!
//! #[derive(Clone)]
//! struct Theme { dark: bool }
//!
//! provide_context(Theme { dark: true });
//! let theme = use_context::<Theme>().unwrap();
//! assert!(theme.dark);
//! ```

use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static CONTEXT_REGISTRY: RefCell<HashMap<TypeId, Box<dyn Any>>>
        = RefCell::new(HashMap::new());
}

/// Register a context value of type `T` for the current scope.
///
/// Calling this again with the same `T` overwrites the previous value.
pub fn provide_context<T: 'static>(value: T) {
    CONTEXT_REGISTRY.with(|reg| {
        reg.borrow_mut().insert(TypeId::of::<T>(), Box::new(value));
    });
}

/// Retrieve the context value of type `T`, if one has been provided.
///
/// Returns a clone of the stored value, or `None` if no context of this
/// type has been registered.
pub fn use_context<T: Clone + 'static>() -> Option<T> {
    CONTEXT_REGISTRY.with(|reg| {
        reg.borrow()
            .get(&TypeId::of::<T>())
            .and_then(|v| v.downcast_ref::<T>())
            .cloned()
    })
}

/// Remove a previously provided context value of type `T`.
///
/// Useful for cleanup in tests and component teardown.
pub fn remove_context<T: 'static>() {
    CONTEXT_REGISTRY.with(|reg| {
        reg.borrow_mut().remove(&TypeId::of::<T>());
    });
}

/// Clear all registered context values.
///
/// Primarily useful in tests to ensure isolation between test cases.
pub fn clear_all_contexts() {
    CONTEXT_REGISTRY.with(|reg| reg.borrow_mut().clear());
}

/// Returns `true` if a context of type `T` is currently registered.
pub fn has_context<T: 'static>() -> bool {
    CONTEXT_REGISTRY.with(|reg| reg.borrow().contains_key(&TypeId::of::<T>()))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Call at start of each test to prevent cross-test pollution.
    fn reset() {
        clear_all_contexts();
    }

    // --- Basic provide / consume ---

    #[test]
    fn test_provide_and_use_context() {
        reset();
        #[derive(Clone, Debug, PartialEq)]
        struct AuthContext {
            user: String,
        }

        provide_context(AuthContext { user: "alice".into() });
        let ctx = use_context::<AuthContext>().unwrap();
        assert_eq!(ctx.user, "alice");
    }

    #[test]
    fn test_use_context_returns_none_when_not_provided() {
        reset();
        #[derive(Clone)]
        struct MissingCtx;
        assert!(use_context::<MissingCtx>().is_none());
    }

    #[test]
    fn test_has_context_true_when_provided() {
        reset();
        #[derive(Clone)]
        struct Flag(bool);
        provide_context(Flag(true));
        assert!(has_context::<Flag>());
    }

    #[test]
    fn test_has_context_false_when_not_provided() {
        reset();
        #[derive(Clone)]
        struct Ghost;
        assert!(!has_context::<Ghost>());
    }

    // --- Overwrite ---

    #[test]
    fn test_provide_overwrites_previous_value() {
        reset();
        #[derive(Clone, PartialEq, Debug)]
        struct Counter(u32);
        provide_context(Counter(1));
        provide_context(Counter(2));
        assert_eq!(use_context::<Counter>().unwrap(), Counter(2));
    }

    // --- Multiple contexts ---

    #[test]
    fn test_multiple_context_types_independent() {
        reset();
        #[derive(Clone, PartialEq, Debug)]
        struct ThemeCtx {
            dark: bool,
        }
        #[derive(Clone, PartialEq, Debug)]
        struct LocaleCtx {
            lang: String,
        }

        provide_context(ThemeCtx { dark: true });
        provide_context(LocaleCtx { lang: "fr".into() });

        assert_eq!(use_context::<ThemeCtx>().unwrap(), ThemeCtx { dark: true });
        assert_eq!(use_context::<LocaleCtx>().unwrap(), LocaleCtx { lang: "fr".into() });
    }

    #[test]
    fn test_many_context_types() {
        reset();
        #[derive(Clone)] struct A(u8);
        #[derive(Clone)] struct B(u16);
        #[derive(Clone)] struct C(u32);
        #[derive(Clone)] struct D(u64);

        provide_context(A(1));
        provide_context(B(2));
        provide_context(C(3));
        provide_context(D(4));

        assert_eq!(use_context::<A>().unwrap().0, 1);
        assert_eq!(use_context::<B>().unwrap().0, 2);
        assert_eq!(use_context::<C>().unwrap().0, 3);
        assert_eq!(use_context::<D>().unwrap().0, 4);
    }

    // --- Remove / cleanup ---

    #[test]
    fn test_remove_context_makes_it_unavailable() {
        reset();
        #[derive(Clone)]
        struct Temp(i32);
        provide_context(Temp(99));
        assert!(use_context::<Temp>().is_some());
        remove_context::<Temp>();
        assert!(use_context::<Temp>().is_none());
    }

    #[test]
    fn test_remove_only_removes_target_type() {
        reset();
        #[derive(Clone, PartialEq, Debug)]
        struct Keep(u8);
        #[derive(Clone)]
        struct Drop(u8);

        provide_context(Keep(1));
        provide_context(Drop(2));
        remove_context::<Drop>();

        assert_eq!(use_context::<Keep>().unwrap(), Keep(1));
        assert!(use_context::<Drop>().is_none());
    }

    #[test]
    fn test_clear_all_removes_everything() {
        reset();
        #[derive(Clone)] struct X;
        #[derive(Clone)] struct Y;
        provide_context(X);
        provide_context(Y);
        clear_all_contexts();
        assert!(!has_context::<X>());
        assert!(!has_context::<Y>());
    }

    // --- Value independence (clone does not alias) ---

    #[test]
    fn test_use_context_returns_clone_not_alias() {
        reset();
        #[derive(Clone, PartialEq, Debug)]
        struct Config {
            value: Vec<i32>,
        }

        provide_context(Config { value: vec![1, 2, 3] });
        let mut ctx = use_context::<Config>().unwrap();
        ctx.value.push(99); // mutate the clone

        // Original in registry should be unchanged
        let original = use_context::<Config>().unwrap();
        assert_eq!(original.value, vec![1, 2, 3]);
    }

    // --- String and primitive context types ---

    #[test]
    fn test_string_context() {
        reset();
        provide_context("hello world".to_string());
        assert_eq!(use_context::<String>().unwrap(), "hello world");
    }

    #[test]
    fn test_numeric_context() {
        reset();
        provide_context(42u32);
        assert_eq!(use_context::<u32>().unwrap(), 42);
    }
}
