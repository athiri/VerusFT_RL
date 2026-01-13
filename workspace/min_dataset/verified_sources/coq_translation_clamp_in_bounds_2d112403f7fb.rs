use vstd::prelude::*;

verus! {

pub open spec fn max_bound_u8() -> nat {
    255
}

pub open spec fn min_bound_u8() -> nat {
    0
}


pub open spec fn clamp_u8(n: int) -> nat {
    if n < min_bound_u8() as int {
        min_bound_u8()
    } else if n > max_bound_u8() as int {
        max_bound_u8()
    } else {
        n as nat
    }
}

pub open spec fn in_bounds_u8(n: nat) -> bool {
    min_bound_u8() <= n && n <= max_bound_u8()
}


pub proof fn clamp_in_bounds(n: int)
    ensures in_bounds_u8(clamp_u8(n))
{
    if n < 0 {
        assert(clamp_u8(n) == 0);
    } else if n > 255 {
        assert(clamp_u8(n) == 255);
    } else {
        assert(clamp_u8(n) == n as nat);
    }
}

} // verus!