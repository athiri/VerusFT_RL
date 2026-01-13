use vstd::prelude::*;

verus! {

pub proof fn mul_add_distr_left(a: nat, b: nat, c: nat)
    ensures a * (b + c) == a * b + a * c
{
    assert(a * (b + c) == a * b + a * c) by (nonlinear_arith);
}

} // verus!