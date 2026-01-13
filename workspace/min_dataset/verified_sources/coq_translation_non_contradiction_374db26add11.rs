use vstd::prelude::*;

verus! {

pub proof fn non_contradiction(p: bool)
    ensures !(p && !p)
{
}

} // verus!