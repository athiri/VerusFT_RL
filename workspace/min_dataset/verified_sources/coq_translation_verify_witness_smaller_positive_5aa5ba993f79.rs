use vstd::prelude::*;

verus! {

pub open spec fn witness_smaller_positive(n: nat) -> nat
    recommends n > 1
{
    1
}


pub proof fn verify_witness_smaller_positive(n: nat)
    requires n > 1
    ensures witness_smaller_positive(n) > 0 && witness_smaller_positive(n) < n
{
}

} // verus!