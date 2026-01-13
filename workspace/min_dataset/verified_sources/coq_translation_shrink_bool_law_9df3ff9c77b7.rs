use vstd::prelude::*;

verus! {

pub open spec fn shrink_bool(b: bool) -> Seq<bool> {
    if b {
        seq![false]
    } else {
        Seq::empty()
    }
}


pub proof fn shrink_bool_law(b: bool, i: int)
    requires 0 <= i < shrink_bool(b).len() as int
    ensures !shrink_bool(b)[i]  // Shrunk values are "smaller" (false < true)
{
    assert(b);  // Only true has shrink candidates
    assert(shrink_bool(true) =~= seq![false]);
    assert(shrink_bool(b)[0] == false);
}

} // verus!