use vstd::prelude::*;

verus! {

pub proof fn lt_irrefl(a: nat)
    ensures !(a < a)
{
}

} // verus!