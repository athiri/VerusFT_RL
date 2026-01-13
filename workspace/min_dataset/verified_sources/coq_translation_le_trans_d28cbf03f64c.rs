use vstd::prelude::*;

verus! {

pub proof fn le_trans(a: nat, b: nat, c: nat)
    requires a <= b, b <= c
    ensures a <= c
{
}

} // verus!