use vstd::prelude::*;

verus! {

pub open spec fn twice(x: int) -> int { x + x }


pub proof fn lemma_twice_commutes(a: int, b: int)
    ensures twice(a + b) == twice(b + a)
{
    // Often, introducing helper functions (like twice) gives the solver
    // a stable function-call term to match on.
    assert(a + b == b + a);
}

} // verus!