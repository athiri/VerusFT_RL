// Source: verification/vstd_extra/src/seq_extra.rs
// Concept: Finding maximum element in a sequence
// This example demonstrates:
// - A spec function defining the maximum of first n elements
// - An executable function finding max with loop invariant
// - The ensures clause guarantees the result is the maximum

use vstd::prelude::*;

verus! {

/// Spec function: compute the maximum of first n elements
/// Base case: max of 1 element is that element
/// Recursive case: max of n elements is larger of max(n-1) and element n-1
pub open spec fn seq_max(s: Seq<u64>, n: int) -> u64
    recommends
        0 < n <= s.len(),
    decreases n,
{
    if n <= 1 {
        s[0]
    } else if s[n - 1] > seq_max(s, n - 1) {
        s[n - 1]
    } else {
        seq_max(s, n - 1)
    }
}

/// Executable function: find maximum element in a non-empty vector
/// Uses loop invariant to track current maximum
pub fn find_max(v: &Vec<u64>) -> (res: u64)
    requires
        v.len() > 0,
    ensures
        res == seq_max(v@, v.len() as int),
        forall|i: int| 0 <= i < v.len() ==> v[i] <= res,
{
    let len: usize = v.len();
    let mut max_val: u64 = v[0];
    let mut idx: usize = 1;
    
    while idx < len
        invariant
            1 <= idx <= len,
            len == v.len(),
            max_val == seq_max(v@, idx as int),
            forall|j: int| 0 <= j < idx ==> v[j] <= max_val,
        decreases len - idx,
    {
        let val: u64 = v[idx];
        if val > max_val {
            max_val = val;
        }
        idx = idx + 1;
    }
    
    max_val
}

fn main() {
    let mut v: Vec<u64> = Vec::new();
    v.push(10);
    v.push(50);
    v.push(30);
    v.push(20);
    
    let max = find_max(&v);
    
    assert(v[0] <= max);
    assert(v[1] <= max);
    assert(v[2] <= max);
    assert(v[3] <= max);
}

} // verus!
