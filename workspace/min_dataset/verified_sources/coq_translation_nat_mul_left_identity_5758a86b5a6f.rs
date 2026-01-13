use vstd::prelude::*;

verus! {

pub open spec fn nat_mul(a: nat, b: nat) -> nat { a * b }

pub open spec fn nat_mul_identity() -> nat { 1 }


pub proof fn nat_mul_left_identity(x: nat)
    ensures nat_mul(nat_mul_identity(), x) == x
{
    assert(1 * x == x) by(nonlinear_arith);
}

} // verus!