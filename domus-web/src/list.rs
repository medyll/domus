//! Keyed list reconciliation for efficient reactive list rendering.
//!
//! The `diff_keys` function and `ListPatch` enum are pure Rust and fully
//! testable without WASM.  The `For<T>` struct wires them up to real DOM
//! and is only compiled on the WASM target.

use std::collections::HashMap;

// ---------------------------------------------------------------------------
// DiffOp — per-new-item operation
// ---------------------------------------------------------------------------

/// What to do for each item in the *new* list.
#[derive(Debug, Clone, PartialEq)]
pub enum DiffOp {
    /// Item exists in old list at `old_idx` — reuse its DOM node.
    Keep(usize),
    /// Item is new — create a fresh DOM node.
    Insert,
}

// ---------------------------------------------------------------------------
// ListPatch — describes the full diff between two keyed lists
// ---------------------------------------------------------------------------

/// The complete set of changes needed to bring the old list up to date.
#[derive(Debug, Clone, PartialEq)]
pub struct ListPatch {
    /// Indices (into old list) of items that must be removed.
    pub removes: Vec<usize>,
    /// Per-position operation for the *new* list.
    pub ops: Vec<DiffOp>,
}

// ---------------------------------------------------------------------------
// diff_keys — pure reconciliation algorithm
// ---------------------------------------------------------------------------

/// Compute the minimal set of operations to transform `old_keys` into
/// `new_keys`.
///
/// Returns a [`ListPatch`] describing which old items to remove and, for
/// each position in the new list, whether to reuse an existing node (`Keep`)
/// or create a new one (`Insert`).
///
/// Time: O(N + M)  Space: O(N)
pub fn diff_keys(old_keys: &[String], new_keys: &[String]) -> ListPatch {
    // Build reverse index: key → old position
    let old_map: HashMap<&str, usize> =
        old_keys.iter().enumerate().map(|(i, k)| (k.as_str(), i)).collect();

    // Build a set of new keys for O(1) "is key still present?" checks
    let new_set: std::collections::HashSet<&str> =
        new_keys.iter().map(String::as_str).collect();

    // Removes: old items whose key no longer appears in the new list
    let removes: Vec<usize> = old_keys
        .iter()
        .enumerate()
        .filter(|(_, k)| !new_set.contains(k.as_str()))
        .map(|(i, _)| i)
        .collect();

    // Per-new-item operation
    let ops: Vec<DiffOp> = new_keys
        .iter()
        .map(|k| {
            if let Some(&old_idx) = old_map.get(k.as_str()) {
                DiffOp::Keep(old_idx)
            } else {
                DiffOp::Insert
            }
        })
        .collect();

    ListPatch { removes, ops }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn keys(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    // --- Basic operations ---

    #[test]
    fn test_empty_to_empty() {
        let patch = diff_keys(&[], &[]);
        assert!(patch.removes.is_empty());
        assert!(patch.ops.is_empty());
    }

    #[test]
    fn test_insert_into_empty() {
        let patch = diff_keys(&[], &keys(&["a", "b", "c"]));
        assert!(patch.removes.is_empty());
        assert_eq!(patch.ops, vec![DiffOp::Insert, DiffOp::Insert, DiffOp::Insert]);
    }

    #[test]
    fn test_remove_all() {
        let patch = diff_keys(&keys(&["a", "b", "c"]), &[]);
        assert_eq!(patch.removes, vec![0, 1, 2]);
        assert!(patch.ops.is_empty());
    }

    // --- Add items ---

    #[test]
    fn test_add_item_to_end() {
        let patch = diff_keys(&keys(&["a", "b"]), &keys(&["a", "b", "c"]));
        assert!(patch.removes.is_empty());
        assert_eq!(patch.ops, vec![
            DiffOp::Keep(0),
            DiffOp::Keep(1),
            DiffOp::Insert,
        ]);
    }

    #[test]
    fn test_add_item_to_start() {
        let patch = diff_keys(&keys(&["b", "c"]), &keys(&["a", "b", "c"]));
        assert!(patch.removes.is_empty());
        assert_eq!(patch.ops, vec![
            DiffOp::Insert,
            DiffOp::Keep(0),
            DiffOp::Keep(1),
        ]);
    }

    #[test]
    fn test_add_multiple_items() {
        let patch = diff_keys(&keys(&["b"]), &keys(&["a", "b", "c", "d"]));
        assert!(patch.removes.is_empty());
        assert_eq!(patch.ops, vec![
            DiffOp::Insert,
            DiffOp::Keep(0),
            DiffOp::Insert,
            DiffOp::Insert,
        ]);
    }

    // --- Remove items ---

    #[test]
    fn test_remove_from_end() {
        let patch = diff_keys(&keys(&["a", "b", "c"]), &keys(&["a", "b"]));
        assert_eq!(patch.removes, vec![2]);
        assert_eq!(patch.ops, vec![DiffOp::Keep(0), DiffOp::Keep(1)]);
    }

    #[test]
    fn test_remove_from_middle() {
        let patch = diff_keys(&keys(&["a", "b", "c"]), &keys(&["a", "c"]));
        assert_eq!(patch.removes, vec![1]);
        assert_eq!(patch.ops, vec![DiffOp::Keep(0), DiffOp::Keep(2)]);
    }

    #[test]
    fn test_remove_multiple_items() {
        let patch = diff_keys(&keys(&["a", "b", "c", "d"]), &keys(&["b"]));
        // a(0), c(2), d(3) are removed
        let mut removes = patch.removes.clone();
        removes.sort();
        assert_eq!(removes, vec![0, 2, 3]);
        assert_eq!(patch.ops, vec![DiffOp::Keep(1)]);
    }

    // --- Reorder items ---

    #[test]
    fn test_reverse_list() {
        let patch = diff_keys(&keys(&["a", "b", "c"]), &keys(&["c", "b", "a"]));
        assert!(patch.removes.is_empty());
        assert_eq!(patch.ops, vec![
            DiffOp::Keep(2),
            DiffOp::Keep(1),
            DiffOp::Keep(0),
        ]);
    }

    #[test]
    fn test_move_first_to_last() {
        let patch = diff_keys(&keys(&["a", "b", "c"]), &keys(&["b", "c", "a"]));
        assert!(patch.removes.is_empty());
        assert_eq!(patch.ops, vec![
            DiffOp::Keep(1),
            DiffOp::Keep(2),
            DiffOp::Keep(0),
        ]);
    }

    #[test]
    fn test_swap_two_items() {
        let patch = diff_keys(&keys(&["a", "b"]), &keys(&["b", "a"]));
        assert!(patch.removes.is_empty());
        assert_eq!(patch.ops, vec![DiffOp::Keep(1), DiffOp::Keep(0)]);
    }

    // --- Combined operations ---

    #[test]
    fn test_add_remove_and_reorder() {
        // old: A B C  →  new: B A D
        let patch = diff_keys(&keys(&["a", "b", "c"]), &keys(&["b", "a", "d"]));
        assert_eq!(patch.removes, vec![2]); // c removed
        assert_eq!(patch.ops, vec![
            DiffOp::Keep(1), // b reused
            DiffOp::Keep(0), // a reused
            DiffOp::Insert,  // d is new
        ]);
    }

    #[test]
    fn test_no_change() {
        let patch = diff_keys(&keys(&["x", "y", "z"]), &keys(&["x", "y", "z"]));
        assert!(patch.removes.is_empty());
        assert_eq!(patch.ops, vec![
            DiffOp::Keep(0),
            DiffOp::Keep(1),
            DiffOp::Keep(2),
        ]);
    }

    // --- Complex / large keys ---

    #[test]
    fn test_numeric_string_keys() {
        let old: Vec<String> = (0..5).map(|i| i.to_string()).collect();
        let new: Vec<String> = (1..6).map(|i| i.to_string()).collect();
        let patch = diff_keys(&old, &new);
        // "0" is removed
        assert!(patch.removes.contains(&0));
        // "5" is inserted
        assert!(patch.ops.last() == Some(&DiffOp::Insert));
        // "1".."4" are kept
        for i in 0..4 {
            assert!(matches!(patch.ops[i], DiffOp::Keep(_)));
        }
    }

    #[test]
    fn test_uuid_style_keys() {
        let old = keys(&["aaa-bbb", "ccc-ddd", "eee-fff"]);
        let new = keys(&["ccc-ddd", "aaa-bbb", "ggg-hhh"]);
        let patch = diff_keys(&old, &new);
        // eee-fff removed
        assert!(patch.removes.contains(&2));
        // ggg-hhh inserted
        assert_eq!(patch.ops[2], DiffOp::Insert);
    }

    // --- Performance: large lists ---

    #[test]
    fn test_large_list_append_100() {
        let old: Vec<String> = (0..1000).map(|i| i.to_string()).collect();
        let new: Vec<String> = (0..1100).map(|i| i.to_string()).collect();
        let patch = diff_keys(&old, &new);
        // No removes, 1000 keeps + 100 inserts
        assert!(patch.removes.is_empty());
        let inserts = patch.ops.iter().filter(|o| **o == DiffOp::Insert).count();
        assert_eq!(inserts, 100);
    }

    #[test]
    fn test_large_list_remove_half() {
        let old: Vec<String> = (0..1000).map(|i| i.to_string()).collect();
        let new: Vec<String> = (0..500).map(|i| i.to_string()).collect();
        let patch = diff_keys(&old, &new);
        assert_eq!(patch.removes.len(), 500);
        assert!(patch.ops.iter().all(|o| matches!(o, DiffOp::Keep(_))));
    }

    #[test]
    fn test_large_list_full_replace() {
        let old: Vec<String> = (0..500).map(|i| format!("old-{}", i)).collect();
        let new: Vec<String> = (0..500).map(|i| format!("new-{}", i)).collect();
        let patch = diff_keys(&old, &new);
        // All old removed, all new inserted
        assert_eq!(patch.removes.len(), 500);
        assert!(patch.ops.iter().all(|o| *o == DiffOp::Insert));
    }

    #[test]
    fn test_patch_insert_count_matches_new_items_not_in_old() {
        let old = keys(&["a", "b", "c"]);
        let new = keys(&["b", "d", "e"]);
        let patch = diff_keys(&old, &new);
        let inserts = patch.ops.iter().filter(|o| **o == DiffOp::Insert).count();
        // d and e are new
        assert_eq!(inserts, 2);
        // a and c are removed
        assert_eq!(patch.removes.len(), 2);
    }
}
