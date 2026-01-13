use vstd::prelude::*;

verus! {

pub proof fn mul_add_distr_right(a: nat, b: nat, c: nat)
    ensures (a + b) * c == a * c + b * c
{
    assert((a + b) * c == a * c + b * c) by (nonlinear_arith);
}

} // verus!