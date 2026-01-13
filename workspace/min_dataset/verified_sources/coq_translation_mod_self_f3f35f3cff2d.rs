use vstd::prelude::*;

verus! {

pub proof fn mod_self(a: nat)
    requires a > 0
    ensures a % a == 0
{
}

} // verus!