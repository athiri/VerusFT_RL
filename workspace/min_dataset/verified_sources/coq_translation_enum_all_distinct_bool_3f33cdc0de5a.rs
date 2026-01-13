use vstd::prelude::*;

verus! {

pub open spec fn enum_all_bool() -> Seq<bool> {
    seq![false, true]
}


pub proof fn enum_all_distinct_bool()
    ensures forall|i: int, j: int| 0 <= i < j < enum_all_bool().len() as int
        ==> enum_all_bool()[i] != enum_all_bool()[j]
{
    assert(enum_all_bool()[0] == false);
    assert(enum_all_bool()[1] == true);
    assert(enum_all_bool()[0] != enum_all_bool()[1]);
}

} // verus!