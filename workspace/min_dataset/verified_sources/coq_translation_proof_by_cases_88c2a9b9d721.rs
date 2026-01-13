use vstd::prelude::*;

verus! {

pub proof fn proof_by_cases(p: bool, q: bool, r: bool)
    requires (p ==> r), (q ==> r)
    ensures (p || q) ==> r
{
}

} // verus!