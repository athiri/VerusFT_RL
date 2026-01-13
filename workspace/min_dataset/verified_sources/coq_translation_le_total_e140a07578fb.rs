use vstd::prelude::*;

verus! {

pub proof fn le_total(a: nat, b: nat)
    ensures a <= b || b <= a
{
}

} // verus!