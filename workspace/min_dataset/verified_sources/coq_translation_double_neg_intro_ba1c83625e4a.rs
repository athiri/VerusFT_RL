use vstd::prelude::*;

verus! {

pub proof fn double_neg_intro(p: bool)
    requires p
    ensures !!p
{
}

} // verus!