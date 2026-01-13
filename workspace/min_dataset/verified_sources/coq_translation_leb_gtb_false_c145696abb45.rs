use vstd::prelude::*;

verus! {

pub open spec fn leb(a: nat, b: nat) -> bool {
    a <= b
}

pub open spec fn gtb(a: nat, b: nat) -> bool {
    a > b
}


pub proof fn leb_gtb_false(a: nat, b: nat)
    requires leb(a, b)
    ensures !gtb(a, b)
{
}

} // verus!