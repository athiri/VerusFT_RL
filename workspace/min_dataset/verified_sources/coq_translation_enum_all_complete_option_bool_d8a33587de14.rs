use vstd::prelude::*;

verus! {

pub open spec fn enum_all_option_bool() -> Seq<Option<bool>> {
    seq![Option::None, Option::Some(false), Option::Some(true)]
}


pub proof fn enum_all_complete_option_bool(o: Option<bool>)
    ensures enum_all_option_bool().contains(o)
{
    match o {
        Option::None => {
            assert(enum_all_option_bool()[0] == Option::<bool>::None);
        }
        Option::Some(b) => {
            if b {
                assert(enum_all_option_bool()[2] == Option::Some(true));
            } else {
                assert(enum_all_option_bool()[1] == Option::Some(false));
            }
        }
    }
}

} // verus!