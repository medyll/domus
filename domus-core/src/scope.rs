use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::effect::Effect;

/// A unique identifier for a Scope.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ScopeId(usize);

pub struct Scope {
    pub id: ScopeId,
    pub effects: Vec<Rc<Effect>>,
    pub parent: Option<ScopeId>,
}

thread_local! {
    static NEXT_SCOPE_ID: AtomicUsize = AtomicUsize::new(1);
    static SCOPES: RefCell<HashMap<ScopeId, Scope>> = RefCell::new(HashMap::new());
}

pub fn create_scope(parent: Option<ScopeId>) -> ScopeId {
    let id = ScopeId(NEXT_SCOPE_ID.with(|c| c.fetch_add(1, Ordering::Relaxed)));
    let scope = Scope { id, effects: Vec::new(), parent };
    SCOPES.with(|scopes| scopes.borrow_mut().insert(id, scope));
    id
}

pub fn dispose_scope(scope_id: ScopeId) {
    SCOPES.with(|scopes| {
        let mut scopes = scopes.borrow_mut();
        scopes.remove(&scope_id);
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::effect::create_effect;
    use crate::signal::signal;

    #[test]
    fn scope_can_be_created_and_disposed() {
        let scope = create_scope(None);
        dispose_scope(scope);
    }
}
