use vstd::prelude::*;

verus! {

pub open spec fn max_bound_u8() -> nat {
    255
}

pub open spec fn min_bound_u8() -> nat {
    0
}


pub proof fn bounded_law_u8()
    ensures min_bound_u8() <= max_bound_u8()
{
    assert(0 <= 255);
}

} // verus!