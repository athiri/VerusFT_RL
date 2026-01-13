use vstd::prelude::*;

verus! {

pub open spec fn prop_double_neg(a: bool) -> bool {
    !!a == a
}


pub proof fn verify_prop_double_neg(a: bool)
    ensures prop_double_neg(a)
{
}

} // verus!