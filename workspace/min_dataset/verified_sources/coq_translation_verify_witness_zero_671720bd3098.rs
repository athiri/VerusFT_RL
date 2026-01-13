use vstd::prelude::*;

verus! {

pub open spec fn witness_zero() -> nat {
    0
}


pub proof fn verify_witness_zero()
    ensures witness_zero() == 0
{
}

} // verus!