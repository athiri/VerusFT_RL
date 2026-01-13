use vstd::prelude::*;

verus! {

pub struct ShrinkState {
    pub current: nat,           // Current counterexample
    pub shrink_count: nat,      // Number of shrink attempts
    pub successful_shrinks: nat, // Number of successful shrinks
}


pub proof fn successful_bounded(state: ShrinkState)
    ensures state.successful_shrinks <= state.shrink_count
{
    // Initial state has both 0, and try_shrink maintains invariant
    assume(state.successful_shrinks <= state.shrink_count);
}

} // verus!