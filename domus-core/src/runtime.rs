use std::cell::RefCell;
use std::rc::Rc;

use crate::effect::Effect;

thread_local! {
    /// Queue of effects to run.
    static EFFECT_QUEUE: RefCell<Vec<Rc<Effect>>> = RefCell::new(Vec::new());
}

/// Schedule an effect to be executed.
///
/// For now this runs the effect immediately; batching will be implemented in
/// later stories.
pub(crate) fn schedule_effect(effect: Rc<Effect>) {
    // Immediately execute for now.
    effect.execute();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::effect::create_effect;
    use crate::signal::signal;

    #[test]
    fn runtime_schedule_runs_effect() {
        let s = signal(0);
        let mut runs = 0;
        let s2 = s.clone();
        create_effect(move || {
            let _ = s2.get();
            runs += 1;
        });
        s.set(1);
        assert!(runs >= 2);
    }
}
