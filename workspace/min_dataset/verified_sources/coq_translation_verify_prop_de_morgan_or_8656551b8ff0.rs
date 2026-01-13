use vstd::prelude::*;

verus! {

pub open spec fn prop_de_morgan_or(a: bool, b: bool) -> bool {
    !(a || b) == (!a && !b)
}


pub proof fn verify_prop_de_morgan_or(a: bool, b: bool)
    ensures prop_de_morgan_or(a, b)
{
}

} // verus!