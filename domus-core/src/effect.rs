use std::cell::RefCell;
use std::rc::Rc;

use crate::runtime;

thread_local! {
    /// The effect currently being executed.
    ///
    /// This is used by `Signal::get` to register dependency tracking.
    pub static RUNNING_EFFECT: RefCell<Option<Rc<Effect>>> = RefCell::new(None);
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
}
