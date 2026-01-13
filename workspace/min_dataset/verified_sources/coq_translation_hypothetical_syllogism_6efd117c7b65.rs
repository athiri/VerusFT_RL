use vstd::prelude::*;

verus! {

pub proof fn hypothetical_syllogism(p: bool, q: bool, r: bool)
    requires p ==> q, q ==> r
    ensures p ==> r
{
}

} // verus!