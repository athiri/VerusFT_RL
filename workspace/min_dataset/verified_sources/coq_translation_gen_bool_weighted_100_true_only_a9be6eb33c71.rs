use vstd::prelude::*;

verus! {

pub open spec fn gen_bool_weighted_outputs(p: nat) -> Set<bool> {
    if p == 0 {
        set![false]
    } else if p >= 100 {
        set![true]
    } else {
        set![true, false]
    }
}


pub proof fn gen_bool_weighted_100_true_only()
    ensures
        gen_bool_weighted_outputs(100).contains(true),
        !gen_bool_weighted_outputs(100).contains(false),
{
}

} // verus!