use vstd::prelude::*;

verus! {

pub open spec fn gen_bool_outputs() -> Set<bool> {
    set![true, false]
}

pub open spec fn gen_bool_not(outputs: Set<bool>) -> Set<bool> {
    Set::new(|b: bool| outputs.contains(!b))
}


pub proof fn gen_bool_not_involutive()
    ensures gen_bool_not(gen_bool_not(gen_bool_outputs())) =~= gen_bool_outputs()
{
    assert forall|b: bool| gen_bool_not(gen_bool_not(gen_bool_outputs())).contains(b)
        implies gen_bool_outputs().contains(b) by {
        // Double negation
    }
    assert forall|b: bool| gen_bool_outputs().contains(b)
        implies gen_bool_not(gen_bool_not(gen_bool_outputs())).contains(b) by {
        assert(gen_bool_outputs().contains(!b));
        assert(gen_bool_not(gen_bool_outputs()).contains(!b));
    }
}

} // verus!