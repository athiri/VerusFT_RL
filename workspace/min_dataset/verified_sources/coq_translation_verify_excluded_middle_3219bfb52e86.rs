use vstd::prelude::*;

verus! {

pub open spec fn excluded_middle(p: bool) -> bool {
    (p || !p) == true
}


pub proof fn verify_excluded_middle(p: bool)
    ensures excluded_middle(p)
{
}

} // verus!