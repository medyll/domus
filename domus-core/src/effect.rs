use std::cell::RefCell;
use std::rc::Rc;

use crate::runtime;

thread_local! {
    /// The effect currently being executed.
    ///
    /// This is used by `Signal::get` to register dependency tracking.
    pub static RUNNING_EFFECT: RefCell<Option<Rc<Effect>>> = RefCell::new(None);
    /// Current execution epoch, incremented when flush starts.
    /// Used to prevent re-entrancy: if an effect tries to re-execute in the same epoch, ignore it.
    static CURRENT_EPOCH: RefCell<u64> = RefCell::new(0);
}

/// A reactive effect that automatically tracks its dependencies.
///
/// Effects are executed immediately when created, and re-run when any
/// of their dependent signals update.
pub struct Effect {
    execute: Box<dyn Fn()>,
}

impl Effect {
    /// Create a new effect and run it once to establish dependencies.
    pub fn new<F: FnMut() + 'static>(f: F) -> Rc<Self> {
        // Wrap a `FnMut` in a `Fn` by storing it in a RefCell and calling
        // it mutably inside a zero-arg `Fn` wrapper. This allows users to
        // pass closures that mutate captured variables while keeping the
        // internal API simple (Box<dyn Fn()>).
        let f_cell = RefCell::new(f);
        let wrapper = move || {
            (f_cell.borrow_mut())();
        };
        let effect = Rc::new(Self { execute: Box::new(wrapper) });
        Self::run(&effect);
        effect
    }

    fn run(effect: &Rc<Self>) {
        // Track the running effect during execution.
        let previous = RUNNING_EFFECT.with(|rt| rt.borrow_mut().replace(Rc::clone(effect)));
        (effect.execute)();
        // Restore previous running effect (if any).
        RUNNING_EFFECT.with(|rt| *rt.borrow_mut() = previous);
    }

    /// Execute the effect once (without changing the TLS context).
    pub fn execute(&self) {
        (self.execute)();
    }

    /// Execute the effect with full dependency tracking and cleanup.
    ///
    /// This clears the effect's previous dependencies, re-runs the effect
    /// with RUNNING_EFFECT set in TLS, and re-establishes dependencies.
    ///
    /// Used by the runtime scheduler to properly re-run effects after signal updates.
    pub fn run_with_dependency_tracking(effect: &Rc<Self>) {
        use crate::signal::unsubscribe_effect_from_all;

        // Clear all previous signal subscriptions.
        // This must be done BEFORE setting RUNNING_EFFECT to avoid borrow conflicts.
        unsubscribe_effect_from_all(effect);

        // Re-run the effect with TLS tracking to establish fresh dependencies
        Self::run(effect);
    }
}

/// Create a new effect, returning a reference-counted handle.
pub fn create_effect<F: FnMut() + 'static>(f: F) -> Rc<Effect> {
    Effect::new(f)
}

/// Schedule an effect to run via the runtime scheduler.
///
/// This is used by Signals to defer execution when batching is enabled.
pub(crate) fn schedule_effect(effect: Rc<Effect>) {
    runtime::schedule_effect(effect);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signal::{signal, Signal};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn effect_runs_immediately_and_tracks() {
        let count = signal(0);
        let runs = Rc::new(RefCell::new(0));

        let count_clone = count.clone();
        let runs_clone = Rc::clone(&runs);
        create_effect(move || {
            // Accessing the signal should register this effect as a subscriber.
            let _ = count_clone.get();
            *runs_clone.borrow_mut() += 1;
        });

        assert_eq!(*runs.borrow(), 1);
    }

    #[test]
    fn effect_dynamic_dependency_switching() {
        // Test that an effect can switch which signals it reads based on conditions,
        // and old subscriptions are properly cleaned up.
        let flag = signal(true);
        let signal_a = signal(1);
        let signal_b = signal(10);
        let values = Rc::new(RefCell::new(Vec::new()));

        let flag_clone = flag.clone();
        let a_clone = signal_a.clone();
        let b_clone = signal_b.clone();
        let values_clone = Rc::clone(&values);

        create_effect(move || {
            let f = flag_clone.get();
            let val = if f {
                a_clone.get()
            } else {
                b_clone.get()
            };
            values_clone.borrow_mut().push(val);
        });

        // Initial run: flag=true, reads a=1
        assert_eq!(*values.borrow(), vec![1]);

        // Change signal_a: effect should re-run (still subscribed to a)
        signal_a.set(2);
        assert_eq!(*values.borrow(), vec![1, 2]);

        // Change signal_b: effect should NOT re-run (not subscribed to b)
        signal_b.set(20);
        assert_eq!(*values.borrow(), vec![1, 2]);

        // Switch flag to false: effect re-runs, now reads b=20
        flag.set(false);
        assert_eq!(*values.borrow(), vec![1, 2, 20]);

        // Change signal_a: effect should NOT re-run anymore (unsubscribed from a)
        signal_a.set(3);
        assert_eq!(*values.borrow(), vec![1, 2, 20]);

        // Change signal_b: effect should re-run (now subscribed to b)
        signal_b.set(21);
        assert_eq!(*values.borrow(), vec![1, 2, 20, 21]);
    }
}
