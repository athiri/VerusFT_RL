use vstd::prelude::*;

verus! {

pub open spec fn shrink_bool(b: bool) -> Seq<bool> {
    if b { seq![false] } else { seq![] }
}


pub proof fn true_shrinks_to_false()
    ensures shrink_bool(true) == seq![false]
{
}

} // verus!