use vstd::prelude::*;

verus! {

pub open spec fn seq_sum(s: Seq<nat>) -> nat {
    fold_left(s, 0, |acc: nat, x: nat| acc + x)
}

pub open spec fn fold_left(s: Seq<nat>, init: nat, op: spec_fn(nat, nat) -> nat) -> nat
    decreases s.len()
{
    if s.len() == 0 {
        init
    } else {
        fold_left(s.skip(1), op(init, s[0]), op)
    }
}


pub proof fn sum_empty()
    ensures seq_sum(Seq::<nat>::empty()) == 0
{
    reveal_with_fuel(fold_left, 2);
}

} // verus!