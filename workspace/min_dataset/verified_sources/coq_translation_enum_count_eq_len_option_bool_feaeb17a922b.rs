use vstd::prelude::*;

verus! {

pub open spec fn enum_count_option_bool() -> nat {
    3
}

pub open spec fn enum_all_option_bool() -> Seq<Option<bool>> {
    seq![Option::None, Option::Some(false), Option::Some(true)]
}


pub proof fn enum_count_eq_len_option_bool()
    ensures enum_count_option_bool() == enum_all_option_bool().len()
{
    assert(enum_all_option_bool().len() == 3);
}

} // verus!