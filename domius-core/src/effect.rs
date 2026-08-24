use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::runtime;

thread_local! {
    /// The effect currently being executed.
    ///
    /// This is used by `Signal::get` to register dependency tracking.
    pub static RUNNING_EFFECT: RefCell<Option<Rc<Effect>>> = const { RefCell::new(None) };
}

/// A reactive effect that automatically tracks its dependencies.
///
/// Effects are executed immediately when created, and re-run when any
/// of their dependent signals update.
///
/// Note: Uses Cell instead of RefCell for the closure to avoid borrow conflicts
/// when effects write signals. Closure state is captured via RefCell in user code.
pub struct Effect {
    /// The closure is wrapped in a Cell to avoid RefCell borrow conflicts.
    /// We use an Option to allow "taking" the closure without holding a borrow.
    execute_fn: Cell<Option<Box<dyn FnMut()>>>,
}

impl Effect {
    /// Create a new effect and run it once to establish dependencies.
    pub fn new<F: FnMut() + 'static>(f: F) -> Rc<Self> {
        let effect = Rc::new(Self {
            execute_fn: Cell::new(Some(Box::new(f))),
        });
        Self::run(&effect);
        effect
    }

    fn run(effect: &Rc<Self>) {
        // Take the closure out of the Cell, execute it, and put it back.
        // This avoids holding a borrow while executing, preventing conflicts.
        // The guard restores the closure and the previously running effect even
        // if the closure unwinds: leaving either behind would silently break
        // dependency tracking for every effect that ran afterwards.
        let Some(f) = effect.execute_fn.take() else {
            return;
        };
        let previous = RUNNING_EFFECT.with(|rt| rt.borrow_mut().replace(Rc::clone(effect)));
        let mut guard = RunGuard {
            effect: Rc::clone(effect),
            closure: Some(f),
            previous: Some(previous),
        };
        if let Some(f) = guard.closure.as_mut() {
            f();
        }
    }

    /// Execute the effect once (without changing the TLS context).
    pub fn execute(&self) {
        if let Some(mut f) = self.execute_fn.take() {
            f();
            self.execute_fn.set(Some(f));
        }
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

/// Puts the runtime back the way it was, whether the effect returned or panicked.
struct RunGuard {
    effect: Rc<Effect>,
    closure: Option<Box<dyn FnMut()>>,
    previous: Option<Option<Rc<Effect>>>,
}

impl Drop for RunGuard {
    fn drop(&mut self) {
        if let Some(closure) = self.closure.take() {
            self.effect.execute_fn.set(Some(closure));
        }
        if let Some(previous) = self.previous.take() {
            RUNNING_EFFECT.with(|rt| *rt.borrow_mut() = previous);
        }
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
    use crate::signal::signal;
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
    fn a_panicking_effect_does_not_break_the_ones_after_it() {
        let poison = signal(0);
        let poison_clone = poison.clone();
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            create_effect(move || {
                poison_clone.get();
                panic!("this effect gives up");
            });
        }));

        // The runtime should be back the way it was, so a later effect still
        // tracks its own signal rather than the abandoned one.
        let source = signal(0);
        let runs = Rc::new(RefCell::new(Vec::new()));
        let watched = source.clone();
        let recorded = Rc::clone(&runs);
        create_effect(move || recorded.borrow_mut().push(watched.get()));

        source.set(1);
        assert_eq!(*runs.borrow(), vec![0, 1]);
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
            let val = if f { a_clone.get() } else { b_clone.get() };
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
