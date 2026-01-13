use vstd::prelude::*;

verus! {

pub open spec fn advance_seed(state: GenState) -> GenState {
    GenState {
        seed: (state.seed * 1103515245 + 12345) % 2147483648,
        size: state.size,
        depth: state.depth,
    }
}


pub struct GenState {
    pub seed: nat,
    pub size: nat,
    pub depth: nat,
}

pub open spec fn split_state(state: GenState) -> (GenState, GenState) {
    let left = advance_seed(state);
    let right = advance_seed(left);
    (left, right)
}


pub proof fn split_produces_different_seeds(state: GenState)
    ensures
        split_state(state).0.seed != state.seed || state.seed == 0,
        split_state(state).1.seed != split_state(state).0.seed || split_state(state).0.seed == 0
{
}

} // verus!