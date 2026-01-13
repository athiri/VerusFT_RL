use vstd::prelude::*;

verus! {

pub open spec fn min_bound_i8() -> int {
    -128
}

pub open spec fn max_bound_i8() -> int {
    127
}


pub proof fn bounded_law_i8()
    ensures min_bound_i8() <= max_bound_i8()
{
    assert(-128 <= 127);
}

} // verus!