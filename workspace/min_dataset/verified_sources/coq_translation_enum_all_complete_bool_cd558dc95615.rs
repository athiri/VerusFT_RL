use vstd::prelude::*;

verus! {

pub open spec fn enum_all_bool() -> Seq<bool> {
    seq![false, true]
}


pub proof fn enum_all_complete_bool(b: bool)
    ensures enum_all_bool().contains(b)
{
    if b {
        assert(enum_all_bool()[1] == true);
        assert(enum_all_bool().contains(true));
    } else {
        assert(enum_all_bool()[0] == false);
        assert(enum_all_bool().contains(false));
    }
}

} // verus!