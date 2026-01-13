use vstd::prelude::*;

verus! {

pub open spec fn gen_bool_outputs() -> Set<bool> {
    set![true, false]
}

pub open spec fn gen_true_outputs() -> Set<bool> {
    set![true]
}

pub open spec fn gen_bool_or(out1: Set<bool>, out2: Set<bool>) -> Set<bool> {
    Set::new(|b: bool| exists|b1: bool, b2: bool|
        out1.contains(b1) && out2.contains(b2) && b == (b1 || b2))
}


pub proof fn gen_bool_or_with_true()
    ensures gen_bool_or(gen_bool_outputs(), gen_true_outputs()) =~= set![true]
{
    assert forall|b: bool| gen_bool_or(gen_bool_outputs(), gen_true_outputs()).contains(b)
        implies b == true by {
        // Any b1 || true == true
    }
    assert(gen_bool_or(gen_bool_outputs(), gen_true_outputs()).contains(true)) by {
        assert(gen_bool_outputs().contains(true));
        assert(gen_true_outputs().contains(true));
    }
}

} // verus!