use vstd::prelude::*;

verus! {

pub open spec fn witness_greater(n: nat) -> nat {
    n + 1
}


pub proof fn verify_witness_greater(n: nat)
    ensures witness_greater(n) > n
{
}

} // verus!