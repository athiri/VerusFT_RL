// Source: verification/vstd_extra/src/function_properties.rs (lines 6-11)
// Concept: A spec function defining injectivity on a domain used in executable code
// Idea: Demonstrates how to define a mathematical property (injectivity) as a spec function
//       and use it to constrain executable code behavior
//
// The spec function `injective_on` checks if a function never maps two distinct
// domain elements to the same image. The executable function `double_values`
// doubles two distinct inputs, preserving distinctness due to injectivity of doubling.

use vstd::prelude::*;

verus! {

pub open spec fn injective_on<A, B>(f: spec_fn(A) -> B, domain: Set<A>) -> bool {
    forall|x: A, y: A|
        #![trigger domain.contains(x), domain.contains(y)]
        domain.contains(x) && domain.contains(y) && f(x) == f(y) ==> x == y
}

pub fn double_values(x: u64, y: u64) -> (res: (u64, u64))
    requires
        x != y,
        x < 0x8000000000000000,
        y < 0x8000000000000000,
        injective_on(|v: u64| (v * 2) as u64, Set::full()),
    ensures
        res.0 == x * 2,
        res.1 == y * 2,
        res.0 != res.1,
{
    (x * 2, y * 2)
}

fn main() {}

} // verus!
