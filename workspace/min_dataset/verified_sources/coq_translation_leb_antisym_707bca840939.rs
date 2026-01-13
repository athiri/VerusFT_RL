use vstd::prelude::*;

verus! {

pub open spec fn leb(a: nat, b: nat) -> bool {
    a <= b
}


pub proof fn leb_antisym(a: nat, b: nat)
    requires leb(a, b), leb(b, a)
    ensures a == b
{
}

} // verus!