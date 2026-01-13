use vstd::prelude::*;

verus! {

pub proof fn add_comm(a: nat, b: nat)
    ensures a + b == b + a
{
}

} // verus!