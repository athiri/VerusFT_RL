use vstd::prelude::*;

verus! {

pub proof fn add_zero_left(a: nat)
    ensures 0 + a == a
{
}

} // verus!