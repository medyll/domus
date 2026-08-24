/// A reactive computed value that automatically re-evaluates when its dependencies change.
///
/// `Computed` wraps a function that reads signals and caches the result.
/// The function is only re-executed when one of its input signals changes,
/// and subscribers are notified of changes to the computed value.
///
/// This is the standard pattern for derived/memo values in reactive systems.
use crate::effect::create_effect;
use crate::scope::{create_effect_in_scope, ScopeId};
use crate::signal::{signal, Signal};

/// A lazily-evaluated reactive value.
///
/// `Computed` is created from a function that reads signals.
/// The function is called once immediately, and again whenever any of
/// its dependencies change. Changes to the computed value notify subscribers.
pub struct Computed<T: Clone + 'static> {
    value: Signal<T>,
}

impl<T: Clone + 'static> Computed<T> {
    /// Create a new computed value from a function.
    ///
    /// The function is executed immediately to establish dependencies
    /// and compute the initial value.
    pub fn new<F: FnMut() -> T + 'static>(mut f: F) -> Self {
        let value = signal(f());

        let value_clone = value.clone();
        create_effect(move || {
            let new_value = f();
            value_clone.set(new_value);
        });

        Self { value }
    }

    /// Create a computed value whose internal effect belongs to `scope_id`.
    ///
    /// Disposing the scope unsubscribes the computation from every source it
    /// observed. `None` means the scope was already gone.
    pub fn new_in_scope<F: FnMut() -> T + 'static>(scope_id: ScopeId, mut f: F) -> Option<Self> {
        let value = signal(f());
        let value_clone = value.clone();
        create_effect_in_scope(scope_id, move || value_clone.set(f()))?;
        Some(Self { value })
    }

    /// Get the current computed value, registering the running effect as a subscriber.
    pub fn get(&self) -> T {
        self.value.get()
    }
}

impl<T: Clone + 'static> Clone for Computed<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}

/// Convenience constructor for computed values.
pub fn computed<T: Clone + 'static, F: FnMut() -> T + 'static>(f: F) -> Computed<T> {
    Computed::new(f)
}

/// Create a computed value disposed together with a reactive scope.
pub fn computed_in_scope<T: Clone + 'static, F: FnMut() -> T + 'static>(
    scope_id: ScopeId,
    f: F,
) -> Option<Computed<T>> {
    Computed::new_in_scope(scope_id, f)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn computed_basic_lazy_evaluation() {
        let source = signal(1);
        let eval_count = Rc::new(RefCell::new(0));

        let source_clone = source.clone();
        let count_clone = Rc::clone(&eval_count);
        let comp = computed(move || {
            *count_clone.borrow_mut() += 1;
            source_clone.get() * 2
        });

        // Initial: computed runs once in new(), then once more in effect setup = 2 evals
        // (This is acceptable as it only happens once per computed creation)
        assert_eq!(comp.get(), 2);
        assert_eq!(*eval_count.borrow(), 2);

        // Change source: computed should re-evaluate (1 more = 3 total)
        source.set(2);
        assert_eq!(comp.get(), 4);
        assert_eq!(*eval_count.borrow(), 3);

        // Read again without changing source: should not re-evaluate
        assert_eq!(comp.get(), 4);
        assert_eq!(*eval_count.borrow(), 3);

        // Change source again
        source.set(3);
        assert_eq!(comp.get(), 6);
        assert_eq!(*eval_count.borrow(), 4);
    }

    #[test]
    fn computed_chain_a_b_c() {
        // A -> B (A*2) -> C (B+1)
        let a = signal(1);

        let a1 = a.clone();
        let b = computed(move || a1.get() * 2);

        let b_clone = b.clone();
        let c = computed(move || b_clone.get() + 1);

        // Initial: a=1, b=2, c=3
        assert_eq!(c.get(), 3);

        // Change a: propagates through chain
        a.set(2);
        assert_eq!(b.get(), 4);
        assert_eq!(c.get(), 5);

        a.set(5);
        assert_eq!(b.get(), 10);
        assert_eq!(c.get(), 11);
    }

    #[test]
    fn computed_shared_no_double_eval() {
        // Two effects read the same computed: it should only re-evaluate once per change
        let source = signal(1);
        let eval_count = Rc::new(RefCell::new(0));

        let source_clone = source.clone();
        let count_clone = Rc::clone(&eval_count);
        let comp = computed(move || {
            *count_clone.borrow_mut() += 1;
            source_clone.get() * 2
        });

        let reads = Rc::new(RefCell::new(Vec::new()));

        // Effect 1 reads computed
        let comp1 = comp.clone();
        let reads1 = Rc::clone(&reads);
        create_effect(move || {
            reads1.borrow_mut().push(comp1.get());
        });

        // Effect 2 reads computed
        let comp2 = comp.clone();
        let reads2 = Rc::clone(&reads);
        create_effect(move || {
            reads2.borrow_mut().push(comp2.get());
        });

        // Initial: computed runs 2 times (new + effect), both external effects see 2
        assert_eq!(*eval_count.borrow(), 2);
        assert_eq!(*reads.borrow(), vec![2, 2]);

        // Change source: computed should evaluate once (1 more = 3 total)
        // Both external effects should have run again with new value
        source.set(2);
        assert_eq!(*eval_count.borrow(), 3);
        assert_eq!(*reads.borrow(), vec![2, 2, 4, 4]);
    }

    #[test]
    fn scoped_computed_stops_evaluating_after_disposal() {
        use crate::scope::{create_scope, dispose_scope};

        let source = signal(1);
        let evaluations = Rc::new(RefCell::new(0));
        let scope = create_scope(None);
        let watched = source.clone();
        let counted = Rc::clone(&evaluations);
        let computed = computed_in_scope(scope, move || {
            *counted.borrow_mut() += 1;
            watched.get() * 2
        })
        .expect("scope should be alive");

        assert_eq!(computed.get(), 2);
        source.set(2);
        assert_eq!(computed.get(), 4);
        let before_disposal = *evaluations.borrow();

        dispose_scope(scope);
        source.set(3);

        assert_eq!(*evaluations.borrow(), before_disposal);
        assert_eq!(computed.get(), 4);
    }
}
