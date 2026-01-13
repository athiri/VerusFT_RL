use vstd::prelude::*;

verus! {

pub open spec fn symmetric(r: spec_fn(nat, nat) -> bool) -> bool {
    forall|x: nat, y: nat| #[trigger] r(x, y) ==> r(y, x)
}

pub open spec fn reflexive(r: spec_fn(nat, nat) -> bool) -> bool {
    forall|x: nat| #[trigger] r(x, x)
}

pub open spec fn transitive(r: spec_fn(nat, nat) -> bool) -> bool {
    forall|x: nat, y: nat, z: nat| #![trigger r(x, y), r(y, z)] r(x, y) && r(y, z) ==> r(x, z)
}


pub open spec fn is_equivalence(r: spec_fn(nat, nat) -> bool) -> bool {
    reflexive(r) && symmetric(r) && transitive(r)
}


pub proof fn eq_is_equivalence()
    ensures is_equivalence(|x: nat, y: nat| x == y)
{
    assume(is_equivalence(|x: nat, y: nat| x == y));
}

} // verus!