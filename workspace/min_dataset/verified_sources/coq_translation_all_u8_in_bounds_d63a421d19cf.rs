use vstd::prelude::*;

verus! {

pub open spec fn max_bound_u8() -> nat {
    255
}

pub open spec fn min_bound_u8() -> nat {
    0
}


pub open spec fn in_bounds_u8(n: nat) -> bool {
    min_bound_u8() <= n && n <= max_bound_u8()
}


pub proof fn all_u8_in_bounds(n: nat)
    requires n <= 255
    ensures in_bounds_u8(n)
{
    assert(0 <= n);
    assert(n <= 255);
}

} // verus!