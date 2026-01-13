use vstd::prelude::*;

verus! {

pub open spec fn nat_min(a: nat, b: nat) -> nat { if a <= b { a } else { b } }


pub proof fn nat_min_assoc(a: nat, b: nat, c: nat)
    ensures nat_min(nat_min(a, b), c) == nat_min(a, nat_min(b, c))
{
}

} // verus!