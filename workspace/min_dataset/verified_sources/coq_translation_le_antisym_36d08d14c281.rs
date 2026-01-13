use vstd::prelude::*;

verus! {

pub proof fn le_antisym(a: nat, b: nat)
    requires a <= b, b <= a
    ensures a == b
{
}

} // verus!