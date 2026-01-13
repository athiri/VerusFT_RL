use vstd::prelude::*;

verus! {

pub proof fn mul_pos(a: nat, b: nat)
    requires a > 0, b > 0
    ensures a * b > 0
{
    assert(a * b > 0) by (nonlinear_arith)
        requires a > 0, b > 0;
}

} // verus!