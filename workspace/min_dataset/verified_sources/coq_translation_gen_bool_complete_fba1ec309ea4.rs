use vstd::prelude::*;

verus! {

pub open spec fn gen_bool_outputs() -> Set<bool> {
    set![true, false]
}


pub proof fn gen_bool_complete()
    ensures
        gen_bool_outputs().contains(true),
        gen_bool_outputs().contains(false),
{
}

} // verus!