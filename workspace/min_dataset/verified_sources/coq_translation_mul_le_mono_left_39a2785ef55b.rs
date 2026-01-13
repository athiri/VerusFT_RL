use vstd::prelude::*;

verus! {

pub proof fn mul_le_mono_left(a: nat, b: nat, c: nat)
    requires b <= c
    ensures a * b <= a * c
{
    assert(a * b <= a * c) by (nonlinear_arith)
        requires b <= c;
}

} // verus!