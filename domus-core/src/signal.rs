use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::effect::Effect;
use crate::effect::schedule_effect;
use crate::effect::RUNNING_EFFECT;

/// Internal non-generic core shared by all signals so we can track them
/// in a global registry and remove subscribers when effects are disposed.
pub struct SignalCore {
    pub subscribers: RefCell<Vec<Rc<Effect>>>,
}

impl SignalCore {
    pub fn new() -> Self {
        Self { subscribers: RefCell::new(Vec::new()) }
    }

    pub fn remove_subscriber(&self, eff: &Rc<Effect>) {
        let mut subs = self.subscribers.borrow_mut();
        subs.retain(|s| !Rc::ptr_eq(s, eff));
    }
}

thread_local! {
    /// Registry of all live signals (weak refs to cores).
    static SIGNAL_REGISTRY: RefCell<Vec<Weak<SignalCore>>> = RefCell::new(Vec::new());
}

/// Unsubscribe an effect from all known signals. Used by scope disposal.
pub(crate) fn unsubscribe_effect_from_all(effect: &Rc<Effect>) {
    SIGNAL_REGISTRY.with(|reg| {
        let mut reg = reg.borrow_mut();
        // Clean dead entries while iterating
        reg.retain(|w| w.upgrade().is_some());
        for weak in reg.iter() {
            if let Some(core) = weak.upgrade() {
                core.remove_subscriber(effect);
            }
        }
    });
}

/// A reactive signal type.
///
/// `Signal` wraps a value and tracks subscribers (effects) that should be
/// re-run when the value changes.
pub struct Signal<T> {
    value: Rc<RefCell<T>>,
    core: Rc<SignalCore>,
}

impl<T: Clone + 'static> Signal<T> {
    /// Create a new signal.
    pub fn new(value: T) -> Self {
        let core = Rc::new(SignalCore::new());
        // Register weak reference for global cleanup
        SIGNAL_REGISTRY.with(|reg| reg.borrow_mut().push(Rc::downgrade(&core)));
        Self {
            value: Rc::new(RefCell::new(value)),
            core,
        }
    }

    /// Get the current value, registering the current effect if one is running.
    pub fn get(&self) -> T {
        RUNNING_EFFECT.with(|rt| {
            if let Some(effect) = rt.borrow().as_ref() {
                let mut subs = self.core.subscribers.borrow_mut();
                if !subs.iter().any(|s| Rc::ptr_eq(s, effect)) {
                    subs.push(Rc::clone(effect));
                }
            }
        });
        self.value.borrow().clone()
    }

    /// Set a new value, notifying all subscribers.
    pub fn set(&self, new_val: T) {
        *self.value.borrow_mut() = new_val;
        for effect in self.core.subscribers.borrow().iter() {
            schedule_effect(Rc::clone(effect));
        }
    }

    /// Mutate the value in place.
    pub fn update<F: FnOnce(&mut T)>(&self, f: F) {
        f(&mut self.value.borrow_mut());
        for effect in self.core.subscribers.borrow().iter() {
            schedule_effect(Rc::clone(effect));
        }
    }

    /// Get the core for testing purposes.
    #[cfg(test)]
    pub fn core(&self) -> &Rc<SignalCore> {
        &self.core
    }
}

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self {
            value: Rc::clone(&self.value),
            core: Rc::clone(&self.core),
        }
    }
}

/// Convenience constructor.
pub fn signal<T: Clone + 'static>(value: T) -> Signal<T> {
    Signal::new(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::effect::create_effect;

    #[test]
    fn signal_get_set_works_and_tracks() {
        let counter = signal(0);
        let runs = Rc::new(RefCell::new(0));

        let counter_clone = counter.clone();
        let runs_clone = Rc::clone(&runs);
        create_effect(move || {
            let _ = counter_clone.get();
            *runs_clone.borrow_mut() += 1;
        });

        assert_eq!(*runs.borrow(), 1);
        counter.set(1);
        // Effects are scheduled; in this simple runtime they run immediately
        assert_eq!(*runs.borrow(), 2);
    }
}
