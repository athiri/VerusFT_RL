use vstd::prelude::*;

verus! {

pub proof fn neg_intro(p: bool)
    requires p ==> false
    ensures !p
{
}

} // verus!