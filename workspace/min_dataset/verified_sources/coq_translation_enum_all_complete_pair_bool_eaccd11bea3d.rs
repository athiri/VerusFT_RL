use vstd::prelude::*;

verus! {

pub open spec fn enum_all_pair_bool() -> Seq<(bool, bool)> {
    seq![
        (false, false),
        (false, true),
        (true, false),
        (true, true)
    ]
}


pub proof fn enum_all_complete_pair_bool(p: (bool, bool))
    ensures enum_all_pair_bool().contains(p)
{
    match p {
        (false, false) => assert(enum_all_pair_bool()[0] == (false, false)),
        (false, true) => assert(enum_all_pair_bool()[1] == (false, true)),
        (true, false) => assert(enum_all_pair_bool()[2] == (true, false)),
        (true, true) => assert(enum_all_pair_bool()[3] == (true, true)),
    }
}

} // verus!