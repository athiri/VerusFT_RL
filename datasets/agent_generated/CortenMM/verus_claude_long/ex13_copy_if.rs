// Source: verification/vstd_extra/src/seq_extra.rs and map_extra.rs
// Concept: Copying elements that satisfy a predicate to a new vector
// This example demonstrates:
// - A spec function filtering a sequence by a predicate (even numbers)
// - An executable function copying matching elements
// - Ensures the result contains exactly the matching elements

use vstd::prelude::*;

verus! {

/// Spec function: filter sequence to keep only even elements
/// Recursively builds a sequence of elements where x % 2 == 0
pub open spec fn filter_even(s: Seq<u64>, n: int) -> Seq<u64>
    recommends
        0 <= n <= s.len(),
    decreases n,
{
    if n <= 0 {
        Seq::empty()
    } else if s[n - 1] % 2 == 0 {
        filter_even(s, n - 1).push(s[n - 1])
    } else {
        filter_even(s, n - 1)
    }
}

/// Executable function: copy even elements to a new vector
/// Iterates through input and appends even elements to result
pub fn copy_even(v: &Vec<u64>) -> (res: Vec<u64>)
    ensures
        res@ == filter_even(v@, v.len() as int),
        forall|i: int| 0 <= i < res.len() ==> res[i] % 2 == 0,
{
    let len: usize = v.len();
    let mut result: Vec<u64> = Vec::new();
    let mut idx: usize = 0;
    
    while idx < len
        invariant
            0 <= idx <= len,
            len == v.len(),
            result@ == filter_even(v@, idx as int),
            forall|i: int| 0 <= i < result.len() ==> result[i] % 2 == 0,
        decreases len - idx,
    {
        let val: u64 = v[idx];
        if val % 2 == 0 {
            result.push(val);
            assert(result@ == filter_even(v@, idx as int).push(val));
        }
        idx = idx + 1;
    }
    
    result
}

fn main() {
    let mut v: Vec<u64> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);
    v.push(6);
    
    let evens = copy_even(&v);
    assert(forall|i: int| 0 <= i < evens.len() ==> evens[i] % 2 == 0);
}

} // verus!

