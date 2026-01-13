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


pub proof fn sum_singleton(x: nat)
    ensures seq_sum(seq![x]) == x
{
    reveal_with_fuel(fold_left, 3);
}

} // verus!