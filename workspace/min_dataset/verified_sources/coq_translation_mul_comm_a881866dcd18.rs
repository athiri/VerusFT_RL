use vstd::prelude::*;

verus! {

pub proof fn mul_comm(a: nat, b: nat)
    ensures a * b == b * a
{
}

} // verus!