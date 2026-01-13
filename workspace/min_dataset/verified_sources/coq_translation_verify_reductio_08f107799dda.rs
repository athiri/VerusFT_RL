use vstd::prelude::*;

verus! {

pub open spec fn reductio(p: bool, q: bool) -> bool {
    ((p ==> q) && (p ==> !q)) ==> !p
}


pub proof fn verify_reductio(p: bool, q: bool)
    ensures reductio(p, q)
{
}

} // verus!