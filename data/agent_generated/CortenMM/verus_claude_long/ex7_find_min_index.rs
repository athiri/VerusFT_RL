// Source: verification/vstd_extra/src/seq_extra.rs
// Concept: Finding the index of minimum element in a sequence
// This example demonstrates:
// - A spec function defining the minimum value in first n elements
// - An executable function finding min index with loop invariant
// - The ensures clause guarantees the index points to a minimum

use vstd::prelude::*;

verus! {

/// Spec function: compute the minimum value in first n elements
/// Base case: min of 1 element is that element
/// Recursive case: min of n elements is smaller of min(n-1) and element n-1
pub open spec fn seq_min(s: Seq<u64>, n: int) -> u64
    recommends
        0 < n <= s.len(),
    decreases n,
{
    if n <= 1 {
        s[0]
    } else if s[n - 1] < seq_min(s, n - 1) {
        s[n - 1]
    } else {
        seq_min(s, n - 1)
    }
}

/// Executable function: find index of minimum element in a non-empty vector
/// Returns the index of the first minimum element found
pub fn find_min_index(v: &Vec<u64>) -> (res: usize)
    requires
        v.len() > 0,
    ensures
        res < v.len(),
        v[res as int] == seq_min(v@, v.len() as int),
        forall|i: int| 0 <= i < v.len() ==> v[res as int] <= v[i],
{
    let len: usize = v.len();
    let mut min_idx: usize = 0;
    let mut min_val: u64 = v[0];
    let mut idx: usize = 1;
    
    while idx < len
        invariant
            1 <= idx <= len,
            len == v.len(),
            min_idx < idx,
            min_val == v[min_idx as int],
            min_val == seq_min(v@, idx as int),
            forall|j: int| 0 <= j < idx ==> min_val <= v[j],
        decreases len - idx,
    {
        let val: u64 = v[idx];
        if val < min_val {
            min_val = val;
            min_idx = idx;
        }
        idx = idx + 1;
    }
    
    min_idx
}

fn main() {
    let mut v: Vec<u64> = Vec::new();
    v.push(50);
    v.push(30);
    v.push(10);
    v.push(40);
    
    let min_idx = find_min_index(&v);
    assert(min_idx < 4);
}

} // verus!

