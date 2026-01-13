use vstd::prelude::*;

verus! {

pub proof fn or_elim(p: bool, q: bool, r: bool)
    requires p || q, p ==> r, q ==> r
    ensures r
{
}

} // verus!