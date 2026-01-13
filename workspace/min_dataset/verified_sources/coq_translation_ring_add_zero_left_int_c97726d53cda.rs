use vstd::prelude::*;

verus! {

pub open spec fn ring_zero_int() -> int {
    0
}

pub open spec fn ring_add_int(a: int, b: int) -> int {
    a + b
}


pub proof fn ring_add_zero_left_int(a: int)
    ensures ring_add_int(ring_zero_int(), a) == a
{
    assert(0 + a == a);
}

} // verus!