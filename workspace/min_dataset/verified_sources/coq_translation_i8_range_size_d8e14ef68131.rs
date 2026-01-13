use vstd::prelude::*;

verus! {

pub open spec fn min_bound_i8() -> int {
    -128
}

pub open spec fn max_bound_i8() -> int {
    127
}


pub proof fn i8_range_size()
    ensures (max_bound_i8() - min_bound_i8() + 1) == 256
{
    assert(127 - (-128) + 1 == 256);
}

} // verus!