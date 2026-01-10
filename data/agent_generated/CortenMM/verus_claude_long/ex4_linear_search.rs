// Source: verification/vstd_extra/src/seq_extra.rs
// Concept: Linear search with loop invariant proving correctness
// This example demonstrates:
// - A spec function defining whether a sequence contains a target
// - An executable linear search with loop invariants proving correctness
// - The ensures clause guarantees correct result

use vstd::prelude::*;

verus! {

/// Spec function: check if a sequence contains a target value
/// Returns true if any element in the sequence equals target
pub open spec fn seq_contains(s: Seq<u64>, target: u64) -> bool {
    exists|i: int| 0 <= i < s.len() && s[i] == target
}

/// Executable function: linear search for target in a vector
/// Returns Some(index) if found, None otherwise
pub fn linear_search(v: &Vec<u64>, target: u64) -> (res: Option<usize>)
    ensures
        match res {
            Some(idx) => idx < v.len() && v[idx as int] == target,
            None => !seq_contains(v@, target),
        },
{
    let len: usize = v.len();
    let mut idx: usize = 0;
    
    while idx < len
        invariant
            0 <= idx <= len,
            len == v.len(),
            forall|j: int| 0 <= j < idx ==> v[j] != target,
        decreases len - idx,
    {
        let val: u64 = v[idx];
        if val == target {
            return Some(idx);
        }
        idx = idx + 1;
    }
    
    assert(!seq_contains(v@, target)) by {
        assert(forall|j: int| 0 <= j < len ==> v[j] != target);
    };
    
    None
}

fn main() {
    let mut v: Vec<u64> = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    v.push(40);
    v.push(50);
    
    let found = linear_search(&v, 30);
    let not_found = linear_search(&v, 99);
    
    assert(!seq_contains(v@, 99));
}

} // verus!

