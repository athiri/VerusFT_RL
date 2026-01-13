use vstd::prelude::*;

verus! {

pub proof fn mod_bound(a: nat, b: nat)
    requires b > 0
    ensures a % b < b
{
}

} // verus!