use vstd::prelude::*;

verus! {

pub open spec fn ltb(a: nat, b: nat) -> bool {
    a < b
}


pub proof fn ltb_trans(a: nat, b: nat, c: nat)
    requires ltb(a, b), ltb(b, c)
    ensures ltb(a, c)
{
}

} // verus!