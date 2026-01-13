use vstd::prelude::*;

verus! {

pub proof fn le_refl(a: nat)
    ensures a <= a
{
}

} // verus!