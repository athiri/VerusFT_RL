// Source: verification/vstd_extra/src/function_properties.rs (lines 51-58)
// Concept: A spec function for defining an inverse operation
// Idea: Demonstrates how to specify the inverse of an operation,
//       selecting the unique preimage for each image element
//
// The spec function `halve_spec` defines the inverse of doubling.
// The executable function `halve` computes the inverse of doubling,
// verified to match the spec.

use vstd::prelude::*;

verus! {

pub open spec fn halve_spec(x: u64) -> u64 {
    (x / 2) as u64
}

pub fn halve(x: u64) -> (res: u64)
    requires
        x % 2 == 0,
    ensures
        res == halve_spec(x),
        res * 2 == x,
{
    x / 2
}

fn main() {}

} // verus!
