use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::effect::Effect;

/// A unique identifier for a Scope.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ScopeId(usize);

pub struct Scope {
    pub id: ScopeId,
    pub effects: Vec<Rc<Effect>>,
    pub parent: Option<ScopeId>,
}

thread_local! {
    static NEXT_SCOPE_ID: AtomicUsize = AtomicUsize::new(1);
    static SCOPES: RefCell<HashMap<ScopeId, Scope>> = RefCell::new(HashMap::new());
}

pub fn create_scope(parent: Option<ScopeId>) -> ScopeId {
    let id = ScopeId(NEXT_SCOPE_ID.with(|c| c.fetch_add(1, Ordering::Relaxed)));
    let scope = Scope { id, effects: Vec::new(), parent };
    SCOPES.with(|scopes| scopes.borrow_mut().insert(id, scope));
    id
}

pub fn dispose_scope(scope_id: ScopeId) {
    use crate::signal::unsubscribe_effect_from_all;

    // Remove scope and unsubscribe its effects from signals.
    SCOPES.with(|scopes| {
        let mut scopes = scopes.borrow_mut();
        if let Some(scope) = scopes.remove(&scope_id) {
            for eff in scope.effects.iter() {
                unsubscribe_effect_from_all(eff);
            }
        }
    });
}

/// Create an effect and register it inside the given scope.
pub fn create_effect_in_scope<F: FnMut() + 'static>(scope_id: ScopeId, f: F) -> Option<Rc<Effect>> {
    use crate::effect::create_effect;

    let eff = create_effect(f);
    SCOPES.with(|scopes| {
        let mut scopes = scopes.borrow_mut();
        if let Some(scope) = scopes.get_mut(&scope_id) {
            scope.effects.push(Rc::clone(&eff));
            Some(eff)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signal::signal;

    #[test]
    fn scope_can_be_created_and_disposed() {
        let scope = create_scope(None);
        dispose_scope(scope);
    }

    #[test]
    fn effects_unsubscribe_on_scope_disposal() {
        let signal = signal(0);
        let scope = create_scope(None);

        let signal_clone = signal.clone();
        let effect = create_effect_in_scope(scope, move || {
            signal_clone.get();
        });

        assert!(effect.is_some());

        // Scope still has subscribers
        let subscribers = signal.core().subscribers.borrow();
        assert_eq!(subscribers.len(), 1);
        drop(subscribers);

        // Dispose scope
        dispose_scope(scope);

        // Effect unsubscribed
        let subscribers = signal.core().subscribers.borrow();
        assert_eq!(subscribers.len(), 0);
    }

    #[test]
    fn no_dangling_references_after_scope_disposal() {
        let signal = signal(42);
        let scope = create_scope(None);

        let signal_clone = signal.clone();
        create_effect_in_scope(scope, move || {
            signal_clone.get();
        });

        // Verify effect is registered
        {
            let subs = signal.core().subscribers.borrow();
            assert_eq!(subs.len(), 1);
        }

        // Dispose the scope
        dispose_scope(scope);

        // Signal should have no subscribers
        {
            let subs = signal.core().subscribers.borrow();
            assert_eq!(subs.len(), 0);
        }

        // Scope should be removed from registry
        SCOPES.with(|scopes| {
            assert!(!scopes.borrow().contains_key(&scope));
        });
    }

    #[test]
    fn parent_child_scope_relationships_maintained() {
        let parent_scope = create_scope(None);
        let child_scope = create_scope(Some(parent_scope));

        // Verify parent is set correctly
        SCOPES.with(|scopes| {
            let scopes = scopes.borrow();
            assert_eq!(scopes.get(&child_scope).unwrap().parent, Some(parent_scope));
        });

        // Dispose child scope
        dispose_scope(child_scope);

        // Parent should still exist
        SCOPES.with(|scopes| {
            let scopes = scopes.borrow();
            assert!(scopes.contains_key(&parent_scope));
            assert!(!scopes.contains_key(&child_scope));
        });

        dispose_scope(parent_scope);
    }

    #[test]
    fn multiple_effects_in_single_scope_unsubscribe_together() {
        let signal1 = signal(1);
        let signal2 = signal(2);
        let scope = create_scope(None);

        let s1 = signal1.clone();
        let e1 = create_effect_in_scope(scope, move || {
            s1.get();
        });
        assert!(e1.is_some());

        let s2 = signal2.clone();
        let e2 = create_effect_in_scope(scope, move || {
            s2.get();
        });
        assert!(e2.is_some());

        // Both signals have 1 subscriber each
        assert_eq!(signal1.core().subscribers.borrow().len(), 1);
        assert_eq!(signal2.core().subscribers.borrow().len(), 1);

        // Dispose scope disposes both effects
        dispose_scope(scope);

        assert_eq!(signal1.core().subscribers.borrow().len(), 0);
        assert_eq!(signal2.core().subscribers.borrow().len(), 0);
    }
}
