use vstd::prelude::*;

verus! {

pub open spec fn ltb(a: nat, b: nat) -> bool {
    a < b
}

pub open spec fn gtb(a: nat, b: nat) -> bool {
    a > b
}

pub open spec fn eqb(a: nat, b: nat) -> bool {
    a == b
}


pub proof fn trichotomy(a: nat, b: nat)
    ensures (ltb(a, b) && !eqb(a, b) && !gtb(a, b)) ||
            (!ltb(a, b) && eqb(a, b) && !gtb(a, b)) ||
            (!ltb(a, b) && !eqb(a, b) && gtb(a, b))
{
}

} // verus!