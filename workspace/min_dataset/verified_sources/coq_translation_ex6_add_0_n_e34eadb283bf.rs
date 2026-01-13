use vstd::prelude::*;

verus! {

pub proof fn ex6_add_0_n(n: nat)
    ensures add(0, n) == n
{
    assert(add(0, n) == n);
}

} // verus!