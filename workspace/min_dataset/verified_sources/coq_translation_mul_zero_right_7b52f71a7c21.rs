use vstd::prelude::*;

verus! {

pub proof fn mul_zero_right(a: nat)
    ensures a * 0 == 0
{
}

} // verus!