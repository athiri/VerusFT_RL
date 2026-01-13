use vstd::prelude::*;

verus! {

pub proof fn mul_zero_left(a: nat)
    ensures 0 * a == 0
{
}

} // verus!