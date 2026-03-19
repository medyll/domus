use std::cell::RefCell;
use std::rc::Rc;

use crate::effect::Effect;
use crate::effect::schedule_effect;
use crate::effect::RUNNING_EFFECT;

/// A reactive signal type.
///
/// `Signal` wraps a value and tracks subscribers (effects) that should be
/// re-run when the value changes.
pub struct Signal<T> {
    value: Rc<RefCell<T>>,
    subscribers: Rc<RefCell<Vec<Rc<Effect>>>>,
}

impl<T: Clone + 'static> Signal<T> {
    /// Create a new signal.
    pub fn new(value: T) -> Self {
        Self {
            value: Rc::new(RefCell::new(value)),
            subscribers: Rc::new(RefCell::new(Vec::new())),
        }
    }

    /// Get the current value, registering the current effect if one is running.
    pub fn get(&self) -> T {
        RUNNING_EFFECT.with(|rt| {
            if let Some(effect) = rt.borrow().as_ref() {
                let mut subs = self.subscribers.borrow_mut();
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
        for effect in self.subscribers.borrow().iter() {
            schedule_effect(Rc::clone(effect));
        }
    }

    /// Mutate the value in place.
    pub fn update<F: FnOnce(&mut T)>(&self, f: F) {
        f(&mut self.value.borrow_mut());
        for effect in self.subscribers.borrow().iter() {
            schedule_effect(Rc::clone(effect));
        }
    }
}

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self {
            value: Rc::clone(&self.value),
            subscribers: Rc::clone(&self.subscribers),
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
        let mut runs = 0;

        let counter_clone = counter.clone();
        create_effect(move || {
            let _ = counter_clone.get();
            runs += 1;
        });

        assert_eq!(runs, 1);
        counter.set(1);
        // Effects are scheduled; in this simple runtime they run immediately
        assert_eq!(runs, 2);
    }
}
