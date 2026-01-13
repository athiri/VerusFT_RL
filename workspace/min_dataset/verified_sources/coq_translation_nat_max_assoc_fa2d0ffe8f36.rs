use vstd::prelude::*;

verus! {

pub open spec fn nat_max(a: nat, b: nat) -> nat { if a >= b { a } else { b } }


pub proof fn nat_max_assoc(a: nat, b: nat, c: nat)
    ensures nat_max(nat_max(a, b), c) == nat_max(a, nat_max(b, c))
{
}

} // verus!