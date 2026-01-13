use vstd::prelude::*;

verus! {

pub proof fn add_le_mono_left(a: nat, b: nat, c: nat)
    requires b <= c
    ensures a + b <= a + c
{
}

} // verus!