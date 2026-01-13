// Source: verification/vstd_extra/src/seq_extra.rs
// Concept: Summing elements of a sequence with loop invariants
// This example demonstrates:
// - A spec function defining sum over a sequence recursively
// - An executable function computing sum with a loop and invariant
// - The ensures clause connects the loop result to the spec definition

use vstd::prelude::*;

verus! {

/// Spec function: compute the sum of first n elements of a sequence
/// Base case: sum of 0 elements is 0
/// Recursive case: sum of first n+1 elements is sum of first n plus element at index n
pub open spec fn seq_sum(s: Seq<u64>, n: int) -> int
    recommends
        0 <= n <= s.len(),
    decreases n,
{
    if n <= 0 {
        0
    } else {
        seq_sum(s, n - 1) + s[n - 1] as int
    }
}

/// Executable function: compute sum of all elements in a vector
/// Uses a loop with invariant connecting partial sum to spec
pub fn compute_sum(v: &Vec<u64>) -> (res: u64)
    requires
        v.len() <= 100,
        forall|i: int| 0 <= i < v.len() ==> v[i] <= 1000,
    ensures
        res as int == seq_sum(v@, v.len() as int),
{
    let mut sum: u64 = 0;
    let mut idx: usize = 0;
    let len: usize = v.len();
    
    while idx < len
        invariant
            0 <= idx <= len,
            len == v.len(),
            len <= 100,
            sum as int == seq_sum(v@, idx as int),
            forall|i: int| 0 <= i < v.len() ==> v[i] <= 1000,
            sum <= idx as u64 * 1000,
        decreases len - idx,
    {
        let val: u64 = v[idx];
        assert(val <= 1000);
        assert(sum <= idx as u64 * 1000);
        assert(idx < 100);
        assert(sum + val <= idx as u64 * 1000 + 1000);
        assert(sum + val <= (idx as u64 + 1) * 1000) by (nonlinear_arith)
            requires
                sum <= idx as u64 * 1000,
                val <= 1000,
                idx < 100,
        ;
        sum = sum + val;
        idx = idx + 1;
    }
    
    sum
}

fn main() {
    let mut v: Vec<u64> = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    v.push(40);
    
    assert(v.len() == 4);
    assert(v@[0] == 10);
    assert(v@[1] == 20);
    assert(v@[2] == 30);
    assert(v@[3] == 40);
    assert(forall|i: int| 0 <= i < v.len() ==> v[i] <= 1000);
    
    let result = compute_sum(&v);
}

} // verus!
