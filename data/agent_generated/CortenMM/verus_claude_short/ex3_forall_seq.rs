// Source: verification/vstd_extra/src/seq_extra.rs (lines 115-118)
// Concept: A spec function for checking if a predicate holds for all sequence elements
// Idea: Demonstrates how to define a universal quantifier over sequence indices as a spec,
//       and use it in executable code to ensure properties hold after sequence operations
//
// The spec function `forall_seq` returns true if predicate f(i, seq[i]) holds for all
// indices i. The executable function `sum_first_two` uses this spec to ensure elements
// are bounded, preventing overflow when computing their sum.

use vstd::prelude::*;

verus! {

pub open spec fn forall_seq<T>(seq: Seq<T>, f: spec_fn(int, T) -> bool) -> bool {
    forall|i| #![trigger seq[i]] 0 <= i < seq.len() ==> f(i, seq[i])
}

pub fn sum_first_two(arr: &[u64]) -> (res: u64)
    requires
        arr@.len() >= 2,
        forall_seq(arr@, |i: int, v: u64| v < 1000000000),
    ensures
        res == arr@[0] + arr@[1],
        res < 2000000000,
{
    let a = arr[0];
    let b = arr[1];
    a + b
}

fn main() {}

} // verus!
