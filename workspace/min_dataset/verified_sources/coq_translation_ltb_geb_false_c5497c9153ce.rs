use vstd::prelude::*;

verus! {

pub open spec fn ltb(a: nat, b: nat) -> bool {
    a < b
}

pub open spec fn geb(a: nat, b: nat) -> bool {
    a >= b
}


pub proof fn ltb_geb_false(a: nat, b: nat)
    requires ltb(a, b)
    ensures !geb(a, b)
{
}

} // verus!