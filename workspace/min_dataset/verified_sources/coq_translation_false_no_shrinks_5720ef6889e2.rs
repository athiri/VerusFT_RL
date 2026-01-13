use vstd::prelude::*;

verus! {

pub open spec fn shrink_bool(b: bool) -> Seq<bool> {
    if b { seq![false] } else { seq![] }
}


pub proof fn false_no_shrinks()
    ensures shrink_bool(false).len() == 0
{
}

} // verus!