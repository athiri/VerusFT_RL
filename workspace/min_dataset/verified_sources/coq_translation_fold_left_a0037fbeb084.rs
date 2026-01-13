use vstd::prelude::*;

verus! {

pub open spec fn fold_left(s: Seq<nat>, init: nat, op: spec_fn(nat, nat) -> nat) -> nat
    decreases s.len()
{
    if s.len() == 0 {
        init
    } else {
        fold_left(s.skip(1), op(init, s[0]), op)
    }
}

} // verus!