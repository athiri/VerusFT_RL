use vstd::prelude::*;

verus! {

pub proof fn lemma_nat_add_comm(a: nat, b: nat)
    ensures a + b == b + a
{
}

} // verus!