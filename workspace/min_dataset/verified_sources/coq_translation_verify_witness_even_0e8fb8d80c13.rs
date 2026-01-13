use vstd::prelude::*;

verus! {

pub open spec fn witness_even() -> nat {
    0
}


pub proof fn verify_witness_even()
    ensures witness_even() % 2 == 0
{
}

} // verus!