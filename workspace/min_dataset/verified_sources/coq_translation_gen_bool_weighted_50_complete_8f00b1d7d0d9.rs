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


pub proof fn gen_bool_weighted_50_complete()
    ensures
        gen_bool_weighted_outputs(50).contains(true),
        gen_bool_weighted_outputs(50).contains(false),
{
}

} // verus!