use vstd::prelude::*;

verus! {

pub struct GenState {
    pub seed: nat,
    pub size: nat,
    pub depth: nat,
}

pub open spec fn advance_seed(state: GenState) -> GenState {
    GenState {
        seed: (state.seed * 1103515245 + 12345) % 2147483648,
        size: state.size,
        depth: state.depth,
    }
}


pub proof fn advance_preserves_size(state: GenState)
    ensures advance_seed(state).size == state.size
{
}

} // verus!