use vstd::prelude::*;

verus! {

pub open spec fn enum_count_pair_bool() -> nat {
    4  // 2 * 2
}

pub open spec fn enum_count_bool() -> nat {
    2
}


pub proof fn enum_count_pair_bool_is_product()
    ensures enum_count_pair_bool() == enum_count_bool() * enum_count_bool()
{
    assert(enum_count_pair_bool() == 4);
    assert(enum_count_bool() == 2);
    assert(2 * 2 == 4);
}

} // verus!