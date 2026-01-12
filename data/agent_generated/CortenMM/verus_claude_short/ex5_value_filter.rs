// Source: verification/vstd_extra/src/map_extra.rs (lines 38-41)
// Concept: A spec function for filtering values based on a predicate
// Idea: Demonstrates how to define filtering at the spec level,
//       useful for selecting entries that satisfy certain conditions
//
// The spec function `passes_filter` checks if a value passes a threshold filter.
// The executable function `filter_value` returns whether a value passes
// the filter threshold.

use vstd::prelude::*;

verus! {

pub open spec fn passes_filter(val: i64, threshold: i64) -> bool {
    val > threshold
}

pub fn filter_value(val: i64, threshold: i64) -> (res: bool)
    ensures
        res == passes_filter(val, threshold),
{
    val > threshold
}

fn main() {}

} // verus!
