// Source: verification/vstd_extra/src/function_properties.rs (lines 13-17)
// Concept: A spec function defining bijectivity between domain and codomain
// Idea: Demonstrates how to express that a function is a one-to-one correspondence,
//       ensuring both injectivity and surjectivity on specified sets
//
// The spec function `is_bijective_pair` checks if two values form a bijective
// pair (different inputs, different outputs). The executable function
// `transform_pair` doubles two distinct values, ensuring distinctness.

use vstd::prelude::*;

verus! {

pub open spec fn is_bijective_pair(x: u64, y: u64, fx: u64, fy: u64) -> bool {
    (x != y) ==> (fx != fy)
}

pub fn transform_pair(x: u64, y: u64) -> (res: (u64, u64))
    requires
        x != y,
        x < 0x8000000000000000,
        y < 0x8000000000000000,
    ensures
        is_bijective_pair(x, y, res.0, res.1),
        res.0 == x * 2,
        res.1 == y * 2,
{
    (x * 2, y * 2)
}

fn main() {}

} // verus!
