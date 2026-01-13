use vstd::prelude::*;

verus! {

pub open spec fn prop_de_morgan_and(a: bool, b: bool) -> bool {
    !(a && b) == (!a || !b)
}


pub proof fn verify_prop_de_morgan_and(a: bool, b: bool)
    ensures prop_de_morgan_and(a, b)
{
}

} // verus!