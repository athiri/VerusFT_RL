use vstd::prelude::*;

verus! {

pub proof fn peirce(p: bool, q: bool)
    ensures ((p ==> q) ==> p) ==> p
{
}

} // verus!