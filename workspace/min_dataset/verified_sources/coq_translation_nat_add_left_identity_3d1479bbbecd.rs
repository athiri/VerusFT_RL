use vstd::prelude::*;

verus! {

pub open spec fn nat_add(a: nat, b: nat) -> nat { a + b }

pub open spec fn nat_add_identity() -> nat { 0 }


pub proof fn nat_add_left_identity(x: nat)
    ensures nat_add(nat_add_identity(), x) == x
{
}

} // verus!