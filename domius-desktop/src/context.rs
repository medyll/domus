//! Context API for Domius desktop applications.
//!
//! Provides process-wide, type-based context storage for desktop windows.

use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Internal context registry.
struct ContextRegistry {
    contexts: HashMap<TypeId, Box<dyn std::any::Any + Send + Sync>>,
}

impl ContextRegistry {
    fn new() -> Self {
        Self {
            contexts: HashMap::new(),
        }
    }
}

// Global context registry using lazy_static pattern
static mut CONTEXT_REGISTRY: Option<Arc<Mutex<ContextRegistry>>> = None;
static CONTEXT_INIT: std::sync::Once = std::sync::Once::new();

fn get_registry() -> Arc<Mutex<ContextRegistry>> {
    unsafe {
        CONTEXT_INIT.call_once(|| {
            CONTEXT_REGISTRY = Some(Arc::new(Mutex::new(ContextRegistry::new())));
        });
        CONTEXT_REGISTRY.as_ref().unwrap().clone()
    }
}

/// Provide a context value.
///
/// Only one value per type can be active at a time.
pub fn provide_context<T: 'static + Send + Sync>(value: T) {
    let registry = get_registry();
    let mut registry = registry.lock().unwrap();
    registry.contexts.insert(TypeId::of::<T>(), Box::new(value));
}

/// Get a context value by type.
///
/// Returns `None` if no context of this type has been provided.
pub fn use_context<T: 'static + Send + Sync + Clone>() -> Option<T> {
    let registry = get_registry();
    let registry = registry.lock().unwrap();
    registry
        .contexts
        .get(&TypeId::of::<T>())
        .and_then(|boxed| boxed.downcast_ref::<T>().cloned())
}

/// Check if a context exists.
pub fn has_context<T: 'static>() -> bool {
    let registry = get_registry();
    let registry = registry.lock().unwrap();
    registry.contexts.contains_key(&TypeId::of::<T>())
}

/// Remove a context.
pub fn remove_context<T: 'static + Send + Sync>() -> Option<T> {
    let registry = get_registry();
    let mut registry = registry.lock().unwrap();
    registry
        .contexts
        .remove(&TypeId::of::<T>())
        .and_then(|boxed| boxed.downcast::<T>().ok())
        .map(|boxed| *boxed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq)]
    struct AppConfig {
        theme: String,
    }

    #[test]
    fn context_provide_and_use() {
        provide_context(AppConfig {
            theme: "dark".into(),
        });

        let config = use_context::<AppConfig>();
        assert!(config.is_some());
        assert_eq!(config.unwrap().theme, "dark");
    }

    #[test]
    fn context_has_context() {
        #[derive(Clone)]
        struct PresenceMarker;

        provide_context(PresenceMarker);

        assert!(has_context::<PresenceMarker>());
        assert!(!has_context::<Vec<i32>>());
    }

    #[test]
    fn context_remove() {
        #[derive(Clone, Debug, PartialEq)]
        struct RemovableConfig {
            theme: String,
        }

        provide_context(RemovableConfig {
            theme: "auto".into(),
        });

        let removed = remove_context::<RemovableConfig>();
        assert_eq!(removed.unwrap().theme, "auto");
        assert!(!has_context::<RemovableConfig>());
    }
}
