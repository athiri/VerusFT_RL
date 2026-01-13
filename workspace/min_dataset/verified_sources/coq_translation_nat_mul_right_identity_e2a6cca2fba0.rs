use vstd::prelude::*;

verus! {

pub open spec fn nat_mul(a: nat, b: nat) -> nat { a * b }

pub open spec fn nat_mul_identity() -> nat { 1 }


pub proof fn nat_mul_right_identity(x: nat)
    ensures nat_mul(x, nat_mul_identity()) == x
{
    assert(x * 1 == x) by(nonlinear_arith);
}

} // verus!