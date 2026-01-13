use vstd::prelude::*;

verus! {

pub struct ShrinkState {
    pub current: nat,           // Current counterexample
    pub shrink_count: nat,      // Number of shrink attempts
    pub successful_shrinks: nat, // Number of successful shrinks
}

pub open spec fn try_shrink(state: ShrinkState, candidate: nat, still_fails: bool) -> ShrinkState {
    if still_fails && candidate < state.current {
        ShrinkState {
            current: candidate,
            shrink_count: state.shrink_count + 1,
            successful_shrinks: state.successful_shrinks + 1,
        }
    } else {
        ShrinkState {
            current: state.current,
            shrink_count: state.shrink_count + 1,
            successful_shrinks: state.successful_shrinks,
        }
    }
}


pub proof fn shrink_count_increases(state: ShrinkState, candidate: nat, still_fails: bool)
    ensures try_shrink(state, candidate, still_fails).shrink_count == state.shrink_count + 1
{
}

} // verus!