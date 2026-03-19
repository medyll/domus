use std::cell::RefCell;
use std::rc::Rc;

use crate::effect::Effect;

thread_local! {
    /// Queue of effects to run during a batch.
    static EFFECT_QUEUE: RefCell<Vec<Rc<Effect>>> = RefCell::new(Vec::new());
    /// Are we currently inside a batch?
    static IN_BATCH: RefCell<bool> = RefCell::new(false);
    /// Has a flush been scheduled (wasm rAF) to flush the queue?
    static FLUSH_SCHEDULED: RefCell<bool> = RefCell::new(false);
}

/// Schedule an effect to be executed. If inside a `batch`, the effect is
/// queued and the queue is flushed later. Outside a batch it runs
/// immediately (or is scheduled for flush on wasm targets).
pub(crate) fn schedule_effect(effect: Rc<Effect>) {
    // If we're in a batch, enqueue (deduped).
    IN_BATCH.with(|in_batch| {
        if *in_batch.borrow() {
            EFFECT_QUEUE.with(|q| {
                let mut q = q.borrow_mut();
                if !q.iter().any(|e| Rc::ptr_eq(e, &effect)) {
                    q.push(effect);
                }
            });
            // On wasm we rely on scheduled flush; on non-wasm we'll flush immediately
            schedule_flush_if_needed();
        } else {
            // Not in a batch: run immediately for non-wasm. On wasm we attempt
            // to schedule a microtask/animation frame to avoid reentrancy.
            #[cfg(target_arch = "wasm32")]
            {
                EFFECT_QUEUE.with(|q| {
                    let mut q = q.borrow_mut();
                    if !q.iter().any(|e| Rc::ptr_eq(e, &effect)) {
                        q.push(effect);
                    }
                });
                schedule_flush_if_needed();
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                effect.execute();
            }
        }
    });
}

fn schedule_flush_if_needed() {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;
        use web_sys::window;

        FLUSH_SCHEDULED.with(|s| {
            if *s.borrow() {
                return;
            }
            *s.borrow_mut() = true;

            // Create a Closure to call `flush_queue` on the next animation frame.
            let cb = Closure::wrap(Box::new(move || {
                // Clear scheduled flag and flush
                FLUSH_SCHEDULED.with(|sf| *sf.borrow_mut() = false);
                flush_queue();
            }) as Box<dyn FnMut()>);

            if let Some(win) = window() {
                // Ignore possible error from request_animation_frame
                let _ = win.request_animation_frame(cb.as_ref().unchecked_ref());
            }

            cb.forget();
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // Non-wasm: flush synchronously
        flush_queue();
    }
}

fn flush_queue() {
    // Drain the queue and execute each effect once, in order.
    let queue = EFFECT_QUEUE.with(|q| q.borrow_mut().drain(..).collect::<Vec<_>>());
    for effect in queue.into_iter() {
        effect.execute();
    }
}

/// Run a closure in a batch context. Effects scheduled during the batch are
/// deduplicated and executed once at the end of the batch (or on next RAF
/// when targeting wasm).
pub fn batch<F: FnOnce()>(f: F) {
    IN_BATCH.with(|b| *b.borrow_mut() = true);
    f();
    IN_BATCH.with(|b| *b.borrow_mut() = false);
    schedule_flush_if_needed();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::effect::create_effect;
    use crate::signal::signal;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn runtime_schedule_runs_effect() {
        let s = signal(0);
        let runs = Rc::new(RefCell::new(0));
        let s2 = s.clone();
        let runs_clone = Rc::clone(&runs);
        create_effect(move || {
            let _ = s2.get();
            *runs_clone.borrow_mut() += 1;
        });
        s.set(1);
        assert!(*runs.borrow() >= 2);
    }
}
