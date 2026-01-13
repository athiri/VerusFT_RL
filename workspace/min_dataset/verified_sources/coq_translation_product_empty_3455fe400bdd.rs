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

pub open spec fn seq_product(s: Seq<nat>) -> nat {
    fold_left(s, 1, |acc: nat, x: nat| acc * x)
}


pub proof fn product_empty()
    ensures seq_product(Seq::<nat>::empty()) == 1
{
    reveal_with_fuel(fold_left, 2);
}

} // verus!