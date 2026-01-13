use vstd::prelude::*;

verus! {

pub open spec fn leb(a: nat, b: nat) -> bool {
    a <= b
}

pub open spec fn ltb(a: nat, b: nat) -> bool {
    a < b
}


pub proof fn ltb_implies_leb(a: nat, b: nat)
    requires ltb(a, b)
    ensures leb(a, b)
{
}

} // verus!