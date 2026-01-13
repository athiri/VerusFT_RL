use vstd::prelude::*;

verus! {

pub struct GenState {
    pub seed: nat,
    pub size: nat,
    pub depth: nat,
}

pub open spec fn increase_depth(state: GenState) -> GenState {
    GenState {
        seed: state.seed,
        size: state.size,
        depth: state.depth + 1,
    }
}


pub proof fn increase_depth_increments(state: GenState)
    ensures increase_depth(state).depth == state.depth + 1
{
}

} // verus!