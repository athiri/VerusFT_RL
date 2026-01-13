// Source: verification/vstd_extra/src/seq_extra.rs and map_extra.rs
// Concept: Counting occurrences of a target value in a sequence
// This example demonstrates:
// - A spec function defining the count of target in first n elements
// - An executable function counting with loop invariant
// - The ensures clause guarantees correct count

use vstd::prelude::*;

verus! {

/// Spec function: count occurrences of target in first n elements
/// Base case: count in 0 elements is 0
/// Recursive case: add 1 if element matches, else add 0
pub open spec fn count_target(s: Seq<u64>, target: u64, n: int) -> nat
    recommends
        0 <= n <= s.len(),
    decreases n,
{
    if n <= 0 {
        0
    } else if s[n - 1] == target {
        count_target(s, target, n - 1) + 1
    } else {
        count_target(s, target, n - 1)
    }
}

/// Executable function: count occurrences of target in a vector
/// Uses loop invariant to maintain running count
pub fn count_occurrences(v: &Vec<u64>, target: u64) -> (res: u64)
    requires
        v.len() <= 10000,
    ensures
        res as nat == count_target(v@, target, v.len() as int),
{
    let len: usize = v.len();
    let mut count: u64 = 0;
    let mut idx: usize = 0;
    
    while idx < len
        invariant
            0 <= idx <= len,
            len == v.len(),
            len <= 10000,
            count as nat == count_target(v@, target, idx as int),
            count <= idx as u64,
        decreases len - idx,
    {
        let val: u64 = v[idx];
        if val == target {
            count = count + 1;
        }
        idx = idx + 1;
    }
    
    count
}

fn main() {
    let mut v: Vec<u64> = Vec::new();
    v.push(10);
    v.push(20);
    v.push(10);
    v.push(30);
    v.push(10);
    
    let count = count_occurrences(&v, 10);
}

} // verus!

