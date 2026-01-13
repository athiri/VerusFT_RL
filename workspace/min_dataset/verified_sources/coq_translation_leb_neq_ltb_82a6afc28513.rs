use vstd::prelude::*;

verus! {

pub open spec fn leb(a: nat, b: nat) -> bool {
    a <= b
}

pub open spec fn ltb(a: nat, b: nat) -> bool {
    a < b
}


pub proof fn leb_neq_ltb(a: nat, b: nat)
    requires leb(a, b), a != b
    ensures ltb(a, b)
{
}

} // verus!