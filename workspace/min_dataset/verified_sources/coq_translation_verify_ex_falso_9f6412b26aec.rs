use vstd::prelude::*;

verus! {

pub open spec fn ex_falso(p: bool) -> bool {
    false ==> p
}


pub proof fn verify_ex_falso(p: bool)
    ensures ex_falso(p)
{
}

} // verus!