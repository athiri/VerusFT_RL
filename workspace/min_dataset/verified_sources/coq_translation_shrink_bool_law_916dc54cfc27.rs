use vstd::prelude::*;

verus! {

pub open spec fn shrink_bool(b: bool) -> Seq<bool> {
    if b {
        seq![false]  // true shrinks to false
    } else {
        Seq::empty()  // false doesn't shrink
    }
}


pub proof fn shrink_bool_law()
    ensures shrink_bool(false).len() == 0,
            shrink_bool(true).len() == 1,
            shrink_bool(true)[0] == false
{
    assert(shrink_bool(false) =~= Seq::empty());
    assert(shrink_bool(true) =~= seq![false]);
}

} // verus!