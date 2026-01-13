use vstd::prelude::*;

verus! {

pub open spec fn fold_right(s: Seq<nat>, init: nat, op: spec_fn(nat, nat) -> nat) -> nat
    decreases s.len()
{
    if s.len() == 0 {
        init
    } else {
        op(s[0], fold_right(s.skip(1), init, op))
    }
}

} // verus!