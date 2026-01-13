use vstd::prelude::*;

verus! {

pub open spec fn nat_mul(a: nat, b: nat) -> nat { a * b }


pub proof fn nat_mul_assoc(a: nat, b: nat, c: nat)
    ensures nat_mul(nat_mul(a, b), c) == nat_mul(a, nat_mul(b, c))
{
    assert(nat_mul(nat_mul(a, b), c) == nat_mul(a, nat_mul(b, c))) by (nonlinear_arith);
}

} // verus!