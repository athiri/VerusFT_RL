use vstd::prelude::*;

verus! {

pub proof fn ex9_mul_0_l(m: nat)
    ensures mul(0, m) == 0
{
    assert(mul(0, m) == 0);
}

} // verus!