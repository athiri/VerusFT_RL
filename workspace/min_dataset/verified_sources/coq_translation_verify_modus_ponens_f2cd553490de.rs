use vstd::prelude::*;

verus! {

pub open spec fn prop_modus_ponens(p: bool, q: bool) -> bool {
    (p && (p ==> q)) ==> q
}


pub proof fn verify_modus_ponens(p: bool, q: bool)
    ensures prop_modus_ponens(p, q)
{
}

} // verus!