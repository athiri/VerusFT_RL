use vstd::prelude::*;

verus! {

pub proof fn dne(p: bool)
    ensures !!p == p
{
}

} // verus!