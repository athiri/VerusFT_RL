use vstd::prelude::*;

verus! {

pub open spec fn nat_add(a: nat, b: nat) -> nat { a + b }


pub proof fn nat_add_assoc(a: nat, b: nat, c: nat)
    ensures nat_add(nat_add(a, b), c) == nat_add(a, nat_add(b, c))
{
}

} // verus!