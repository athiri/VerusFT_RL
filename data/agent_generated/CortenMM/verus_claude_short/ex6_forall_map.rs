// Source: verification/vstd_extra/src/map_extra.rs (lines 266-269)
// Concept: A spec function for checking if a predicate holds for all elements
// Idea: Demonstrates universal quantification over elements,
//       used to express invariants about array contents
//
// The spec function `all_below` checks if all elements are below a threshold.
// The executable function `check_first_two_below` checks if the first two
// elements are below the threshold.

use vstd::prelude::*;

verus! {

pub open spec fn all_below(s: Seq<u64>, threshold: u64) -> bool {
    forall|i: int| 0 <= i < s.len() ==> s[i] < threshold
}

pub fn check_first_two_below(arr: &[u64], threshold: u64) -> (res: bool)
    requires
        arr@.len() >= 2,
        all_below(arr@, threshold),
    ensures
        res == (arr@[0] < threshold && arr@[1] < threshold),
{
    arr[0] < threshold && arr[1] < threshold
}

fn main() {}

} // verus!
