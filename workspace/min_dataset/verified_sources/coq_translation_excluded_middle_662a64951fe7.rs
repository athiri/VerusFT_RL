use vstd::prelude::*;

verus! {

pub proof fn excluded_middle(p: bool)
    ensures p || !p
{
}

} // verus!