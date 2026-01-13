use vstd::prelude::*;

verus! {

pub proof fn or_intro_right(p: bool, q: bool)
    requires q
    ensures p || q
{
}

} // verus!