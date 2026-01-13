use vstd::prelude::*;

verus! {

pub open spec fn exists_bounded(bound: nat, pred: spec_fn(nat) -> bool) -> bool {
    exists|x: nat| x < bound && #[trigger] pred(x)
}


pub proof fn bounded_existence_from_witness(n: nat, bound: nat, pred: spec_fn(nat) -> bool)
    requires n < bound, pred(n)
    ensures exists_bounded(bound, pred)
{
}

} // verus!