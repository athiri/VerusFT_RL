use vstd::prelude::*;

verus! {

pub open spec fn gen_bool_outputs() -> Set<bool> {
    set![true, false]
}

pub open spec fn gen_false_outputs() -> Set<bool> {
    set![false]
}

pub open spec fn gen_bool_and(out1: Set<bool>, out2: Set<bool>) -> Set<bool> {
    Set::new(|b: bool| exists|b1: bool, b2: bool|
        out1.contains(b1) && out2.contains(b2) && b == (b1 && b2))
}


pub proof fn gen_bool_and_with_false()
    ensures gen_bool_and(gen_bool_outputs(), gen_false_outputs()) =~= set![false]
{
    assert forall|b: bool| gen_bool_and(gen_bool_outputs(), gen_false_outputs()).contains(b)
        implies b == false by {
        // Any b1 && false == false
    }
    assert(gen_bool_and(gen_bool_outputs(), gen_false_outputs()).contains(false)) by {
        assert(gen_bool_outputs().contains(true));
        assert(gen_false_outputs().contains(false));
    }
}

} // verus!