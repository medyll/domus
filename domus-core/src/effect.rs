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
    pub fn new<F: Fn() + 'static>(f: F) -> Rc<Self> {
        let effect = Rc::new(Self { execute: Box::new(f) });
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
pub fn create_effect<F: Fn() + 'static>(f: F) -> Rc<Effect> {
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

    #[test]
    fn effect_runs_immediately_and_tracks() {
        let count = signal(0);
        let mut runs = 0;

        let count_clone = count.clone();
        create_effect(move || {
            // Accessing the signal should register this effect as a subscriber.
            let _ = count_clone.get();
            runs += 1;
        });

        assert_eq!(runs, 1);
    }
}
