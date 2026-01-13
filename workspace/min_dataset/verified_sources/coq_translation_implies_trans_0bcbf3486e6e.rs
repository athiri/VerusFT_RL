use vstd::prelude::*;

verus! {

pub proof fn implies_trans(p: bool, q: bool, r: bool)
    requires p ==> q, q ==> r
    ensures p ==> r
{
}

} // verus!