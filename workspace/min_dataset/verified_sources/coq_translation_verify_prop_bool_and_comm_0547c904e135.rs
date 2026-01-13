use vstd::prelude::*;

verus! {

pub open spec fn prop_bool_and_comm(a: bool, b: bool) -> bool {
    (a && b) == (b && a)
}


pub proof fn verify_prop_bool_and_comm(a: bool, b: bool)
    ensures prop_bool_and_comm(a, b)
{
}

} // verus!