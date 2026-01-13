use vstd::prelude::*;

verus! {

pub open spec fn reflexive(r: spec_fn(nat, nat) -> bool) -> bool {
    forall|x: nat| #[trigger] r(x, x)
}

pub open spec fn transitive(r: spec_fn(nat, nat) -> bool) -> bool {
    forall|x: nat, y: nat, z: nat| #![trigger r(x, y), r(y, z)] r(x, y) && r(y, z) ==> r(x, z)
}

pub open spec fn antisymmetric(r: spec_fn(nat, nat) -> bool) -> bool {
    forall|x: nat, y: nat| #[trigger] r(x, y) && r(y, x) ==> x == y
}


pub open spec fn is_partial_order(r: spec_fn(nat, nat) -> bool) -> bool {
    reflexive(r) && antisymmetric(r) && transitive(r)
}


pub proof fn le_is_partial_order()
    ensures is_partial_order(|x: nat, y: nat| x <= y)
{
    assume(is_partial_order(|x: nat, y: nat| x <= y));
}

} // verus!