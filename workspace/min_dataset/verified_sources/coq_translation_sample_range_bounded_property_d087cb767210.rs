use vstd::prelude::*;

verus! {

pub type Seed = nat;

pub struct RandomState {
    pub seed: Seed,
    pub generated_count: nat,
}

pub open spec fn next_value(state: RandomState) -> (nat, RandomState) {
    let new_seed = (state.seed * 1103515245 + 12345) % 2147483648;
    (new_seed, RandomState { seed: new_seed, generated_count: state.generated_count + 1 })
}

pub open spec fn sample_range(state: RandomState, max: nat) -> (nat, RandomState)
{
    if max == 0 {
        (0, state)
    } else {
        let (v, new_state) = next_value(state);
        (v % max, new_state)
    }
}


pub proof fn sample_range_bounded_property(state: RandomState, max: nat)
    requires max > 0
    ensures sample_range(state, max).0 < max
{
    let (v, new_state) = next_value(state);
    assert(v % max < max) by(nonlinear_arith)
        requires max > 0;
}

} // verus!