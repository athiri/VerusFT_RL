use vstd::prelude::*;

verus! {

pub proof fn add_le_mono_right(a: nat, b: nat, c: nat)
    requires a <= b
    ensures a + c <= b + c
{
}

} // verus!