// Source: verification/vstd_extra/src/seq_extra.rs and function_properties.rs
// Concept: Binary search with loop invariant proving correctness
// This example demonstrates:
// - A spec function defining whether a sequence is sorted
// - An executable binary search with loop invariants proving correctness
// - The ensures clause guarantees the result is correct if found

use vstd::prelude::*;

verus! {

/// Spec function: check if a sequence is sorted in non-decreasing order
/// A sequence is sorted if for all adjacent pairs, the first is <= second
pub open spec fn is_sorted(s: Seq<u64>) -> bool {
    forall|i: int, j: int| 0 <= i < j < s.len() ==> s[i] <= s[j]
}

/// Executable function: binary search for target in a sorted vector
/// Returns Some(index) if found, None otherwise
/// Uses loop invariant to maintain search bounds correctness
pub fn binary_search(v: &Vec<u64>, target: u64) -> (res: Option<usize>)
    requires
        is_sorted(v@),
        v.len() < usize::MAX / 2,
    ensures
        match res {
            Some(idx) => idx < v.len() && v[idx as int] == target,
            None => forall|i: int| 0 <= i < v.len() ==> v[i as int] != target,
        },
{
    let len: usize = v.len();
    if len == 0 {
        return None;
    }
    
    let mut lo: usize = 0;
    let mut hi: usize = len;
    
    while lo < hi
        invariant
            0 <= lo <= hi <= len,
            len == v.len(),
            is_sorted(v@),
            forall|i: int| 0 <= i < lo ==> v[i as int] < target,
            forall|i: int| hi <= i < len ==> v[i as int] > target,
        decreases hi - lo,
    {
        let mid: usize = lo + (hi - lo) / 2;
        let mid_val: u64 = v[mid];
        
        if mid_val == target {
            return Some(mid);
        } else if mid_val < target {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    
    None
}

fn main() {
    let mut v: Vec<u64> = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    v.push(40);
    v.push(50);
    
    assert(v.len() == 5);
    assert(is_sorted(v@));
    
    let result1 = binary_search(&v, 30);
    let result2 = binary_search(&v, 25);
    let result3 = binary_search(&v, 10);
    let result4 = binary_search(&v, 50);
    let result5 = binary_search(&v, 100);
}

} // verus!
