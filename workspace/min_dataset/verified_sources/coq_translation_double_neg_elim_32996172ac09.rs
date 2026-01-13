use vstd::prelude::*;

verus! {

pub proof fn double_neg_elim(p: bool)
    requires !!p
    ensures p
{
}

} // verus!