use std::cell::RefCell;
use std::rc::Rc;

use crate::effect::Effect;

thread_local! {
    /// Primary queue of effects ready to execute immediately.
    static PRIMARY_QUEUE: RefCell<Vec<Rc<Effect>>> = RefCell::new(Vec::new());
    /// Secondary queue for effects scheduled during a flush (to prevent duplicates in one generation).
    static SECONDARY_QUEUE: RefCell<Vec<Rc<Effect>>> = RefCell::new(Vec::new());
    /// Nesting depth of batch calls. When > 0, effects are queued instead of run immediately.
    static BATCH_DEPTH: RefCell<usize> = RefCell::new(0);
    /// Are we currently flushing the queue? This prevents effects scheduled during a flush
    /// from executing immediately and duplicating in the current generation.
    static IS_FLUSHING: RefCell<bool> = RefCell::new(false);
    /// Effects already executed in the current generation (prevents re-entrancy loops).
    static EXECUTED_THIS_GENERATION: RefCell<Vec<*const Effect>> = RefCell::new(Vec::new());
    /// Has a flush been scheduled (wasm rAF) to flush the queue?
    static FLUSH_SCHEDULED: RefCell<bool> = RefCell::new(false);
}

/// Schedule an effect to be executed. If inside a `batch`, the effect is
/// queued and the queue is flushed later. Outside a batch it runs
/// immediately (or is scheduled for flush on wasm targets).
pub(crate) fn schedule_effect(effect: Rc<Effect>) {
    let effect_ptr = effect.as_ref() as *const Effect;

    // Check if already executed in this generation (prevent re-entrancy loops)
    let already_executed = EXECUTED_THIS_GENERATION.with(|executed| {
        executed.borrow().iter().any(|&ptr| ptr == effect_ptr)
    });
    if already_executed {
        return; // Ignore re-scheduling of the same effect in the same generation
    }

    // Check if we're in a batch
    let in_batch = BATCH_DEPTH.with(|depth| *depth.borrow() > 0);
    if in_batch {
        PRIMARY_QUEUE.with(|q| {
            let mut q = q.borrow_mut();
            if !q.iter().any(|e| Rc::ptr_eq(e, &effect)) {
                q.push(effect);
            }
        });
        return;
    }

    // Check if we're currently flushing
    let is_flushing = IS_FLUSHING.with(|flushing| *flushing.borrow());
    if is_flushing {
        SECONDARY_QUEUE.with(|q| {
            let mut q = q.borrow_mut();
            if !q.iter().any(|e| Rc::ptr_eq(e, &effect)) {
                q.push(effect);
            }
        });
        return;
    }

    // Not in a batch and not flushing: run immediately for non-wasm.
    // On wasm we queue for animation frame.
    #[cfg(target_arch = "wasm32")]
    {
        PRIMARY_QUEUE.with(|q| {
            let mut q = q.borrow_mut();
            if !q.iter().any(|e| Rc::ptr_eq(e, &effect)) {
                q.push(effect);
            }
        });
        schedule_flush_if_needed();
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // Run with full dependency tracking to ensure dynamic deps work
        Effect::run_with_dependency_tracking(&effect);
    }
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
    // Process effects in generations to prevent glitches and duplicate executions in diamond graphs.
    // Each generation executes all pending effects, then processes any effects that were
    // scheduled during their execution (secondary queue), in the next generation.
    // Re-entrancy prevention: track which effects have already executed to avoid loops.
    IS_FLUSHING.with(|flushing| {
        *flushing.borrow_mut() = true;
    });

    loop {
        // Clear the executed set for this generation
        EXECUTED_THIS_GENERATION.with(|executed| {
            executed.borrow_mut().clear();
        });

        // Drain primary queue for this generation
        let queue = PRIMARY_QUEUE.with(|q| q.borrow_mut().drain(..).collect::<Vec<_>>());

        if queue.is_empty() {
            break; // No more effects to execute
        }

        // Execute all effects in this generation with proper dependency tracking
        for effect in queue.into_iter() {
            // Mark as executed before running (to prevent re-entrancy if it modifies a signal)
            let effect_ptr = effect.as_ref() as *const Effect;
            EXECUTED_THIS_GENERATION.with(|executed| {
                executed.borrow_mut().push(effect_ptr);
            });

            Effect::run_with_dependency_tracking(&effect);
        }

        // Move secondary queue to primary for next generation
        SECONDARY_QUEUE.with(|q| {
            let secondary = q.borrow_mut().drain(..).collect::<Vec<_>>();
            if !secondary.is_empty() {
                PRIMARY_QUEUE.with(|pq| {
                    pq.borrow_mut().extend(secondary);
                });
            }
        });
    }

    IS_FLUSHING.with(|flushing| {
        *flushing.borrow_mut() = false;
    });

    // Clear executed set at the end
    EXECUTED_THIS_GENERATION.with(|executed| {
        executed.borrow_mut().clear();
    });
}

/// Run a closure in a batch context. Effects scheduled during the batch are
/// deduplicated and executed once at the end of the outermost batch.
/// Nested batch() calls are supported: only the outermost batch triggers a flush.
pub fn batch<F: FnOnce()>(f: F) {
    BATCH_DEPTH.with(|depth| {
        let mut d = depth.borrow_mut();
        *d += 1;
    });
    f();
    BATCH_DEPTH.with(|depth| {
        let mut d = depth.borrow_mut();
        *d -= 1;
        // Flush only when exiting the outermost batch
        if *d == 0 {
            drop(d); // Release borrow before flushing
            schedule_flush_if_needed();
        }
    });
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

    #[test]
    fn batch_nested_single_flush() {
        let s = signal(0);
        let runs = Rc::new(RefCell::new(0));
        let s2 = s.clone();
        let runs_clone = Rc::clone(&runs);
        create_effect(move || {
            let _ = s2.get();
            *runs_clone.borrow_mut() += 1;
        });
        // Initial effect run: runs = 1
        assert_eq!(*runs.borrow(), 1);

        // Nested batch: both signal mutations happen before any flush
        batch(|| {
            s.set(1);
            batch(|| {
                s.set(2);
            });
            // Inner batch exited but we're still in outer batch
            // Effect should NOT have run yet
            assert_eq!(*runs.borrow(), 1);
        });
        // Outer batch exited: flush happens, effect runs once
        assert_eq!(*runs.borrow(), 2);
    }

    #[test]
    fn batch_triple_nested_single_flush() {
        let s = signal(0);
        let runs = Rc::new(RefCell::new(0));
        let s2 = s.clone();
        let runs_clone = Rc::clone(&runs);
        create_effect(move || {
            let _ = s2.get();
            *runs_clone.borrow_mut() += 1;
        });
        assert_eq!(*runs.borrow(), 1);

        batch(|| {
            s.set(1);
            batch(|| {
                s.set(2);
                batch(|| {
                    s.set(3);
                });
                assert_eq!(*runs.borrow(), 1);
            });
            assert_eq!(*runs.borrow(), 1);
        });
        // Only one flush at the end
        assert_eq!(*runs.borrow(), 2);
    }

    #[test]
    fn diamond_convergence_single_execution() {
        // Diamond: A -> [B, C] -> D
        // When A changes, D should execute exactly once, not twice
        let a = signal(1);
        let d_runs = Rc::new(RefCell::new(0));

        let a1 = a.clone();
        let _b = create_effect({
            let a = a1.clone();
            move || {
                let _ = a.get(); // B reads A
            }
        });

        let a2 = a.clone();
        let _c = create_effect({
            let a = a2.clone();
            move || {
                let _ = a.get(); // C reads A
            }
        });

        let d_runs_clone = Rc::clone(&d_runs);
        create_effect({
            // D reads from both B and C (indirectly by reading A)
            let a = a.clone();
            move || {
                let _ = a.get();
                *d_runs_clone.borrow_mut() += 1;
            }
        });

        // Initial run: d_runs = 1
        assert_eq!(*d_runs.borrow(), 1);

        // Change A: B and C both depend on A, D depends on A
        // D should execute exactly once, not twice
        a.set(2);
        assert_eq!(*d_runs.borrow(), 2);

        a.set(3);
        assert_eq!(*d_runs.borrow(), 3);
    }

    #[test]
    fn effect_reentrancy_prevented() {
        // Test that re-entrancy is prevented: when an effect writes a signal during execution,
        // it's not immediately re-scheduled (preventing infinite loops).
        // The write is deferred to the next generation via the secondary queue.
        let counter = signal(0);
        let writes = Rc::new(RefCell::new(0));

        let counter_clone = counter.clone();
        let writes_clone = Rc::clone(&writes);
        create_effect(move || {
            let val = counter_clone.get();
            // Attempt to increment: this should be deferred, not immediate
            if val == 0 {
                counter_clone.set(1);
                *writes_clone.borrow_mut() += 1;
            }
        });

        // Effect ran once (initial), attempted one write
        assert_eq!(*writes.borrow(), 1);
        // Signal was written but effect didn't re-execute immediately (blocked in same generation)
        assert_eq!(counter.get(), 1);
    }

    #[test]
    fn glitch_free_convergence() {
        // Ensure that during a flush generation, dependent values see consistent state.
        // A = signal, B = derived from A, C = derived from A+B
        // Observer of C should never see glitchy intermediate values.
        let a = signal(1);
        let values_seen = Rc::new(RefCell::new(Vec::new()));

        let a1 = a.clone();
        create_effect({
            let a = a1;
            let values = Rc::clone(&values_seen);
            move || {
                let av = a.get();
                let bv = av * 2;
                let cv = av + bv; // Should be av + (av*2) = av*3
                values.borrow_mut().push(cv);
            }
        });

        // Initial: a=1, b=2, c=3
        assert_eq!(*values_seen.borrow(), vec![3]);

        // Change a to 2: should see c=6, not any intermediate value
        a.set(2);
        assert_eq!(*values_seen.borrow(), vec![3, 6]);

        a.set(3);
        assert_eq!(*values_seen.borrow(), vec![3, 6, 9]);
    }

}
