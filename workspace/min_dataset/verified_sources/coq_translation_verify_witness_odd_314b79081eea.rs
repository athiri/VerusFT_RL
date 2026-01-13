use vstd::prelude::*;

verus! {

pub open spec fn witness_odd() -> nat {
    1
}


pub proof fn verify_witness_odd()
    ensures witness_odd() % 2 == 1
{
}

} // verus!