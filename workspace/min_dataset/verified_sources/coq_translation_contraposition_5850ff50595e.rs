use vstd::prelude::*;

verus! {

pub proof fn contraposition(p: bool, q: bool)
    ensures (p ==> q) <==> (!q ==> !p)
{
}

} // verus!