use vstd::prelude::*;

verus! {

pub proof fn existence_from_witness(n: nat, pred: spec_fn(nat) -> bool)
    requires pred(n)
    ensures exists|x: nat| #[trigger] pred(x)
{
}

} // verus!