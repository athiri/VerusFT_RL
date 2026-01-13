use vstd::prelude::*;

verus! {

pub open spec fn enum_all_bool() -> Seq<bool> {
    seq![false, true]
}

pub open spec fn enum_count_bool() -> nat {
    2
}


pub proof fn enum_count_eq_len_bool()
    ensures enum_count_bool() == enum_all_bool().len()
{
    assert(enum_all_bool().len() == 2);
    assert(enum_count_bool() == 2);
}

} // verus!