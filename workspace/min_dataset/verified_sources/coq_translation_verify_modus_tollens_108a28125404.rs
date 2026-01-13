use vstd::prelude::*;

verus! {

pub open spec fn prop_modus_tollens(p: bool, q: bool) -> bool {
    (!q && (p ==> q)) ==> !p
}


pub proof fn verify_modus_tollens(p: bool, q: bool)
    ensures prop_modus_tollens(p, q)
{
}

} // verus!