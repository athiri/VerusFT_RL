use vstd::prelude::*;

verus! {

pub open spec fn next_value(state: RandomState) -> (nat, RandomState) {
    let new_seed = (state.seed * 1103515245 + 12345) % 2147483648;
    (new_seed, RandomState { seed: new_seed, generated_count: state.generated_count + 1 })
}


pub type Seed = nat;

pub struct RandomState {
    pub seed: Seed,
    pub generated_count: nat,
}

pub open spec fn sample_range_bounded(state: RandomState, lo: nat, hi: nat) -> (nat, RandomState)
{
    if hi <= lo {
        (lo, state)
    } else {
        let (v, new_state) = sample_range(state, (hi - lo) as nat);
        (lo + v, new_state)
    }
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


pub proof fn sample_bounded_in_range(state: RandomState, lo: nat, hi: nat)
    requires lo < hi
    ensures
        sample_range_bounded(state, lo, hi).0 >= lo,
        sample_range_bounded(state, lo, hi).0 < hi
{
    let (v, _) = sample_range(state, (hi - lo) as nat);
    assume(v < hi - lo);  // From sample_range_bounded_property
}

} // verus!