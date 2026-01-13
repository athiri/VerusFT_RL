use vstd::prelude::*;

verus! {

pub open spec fn ring_zero_int() -> int {
    0
}

pub open spec fn ring_mul_int(a: int, b: int) -> int {
    a * b
}


pub proof fn ring_mul_zero_left_int(a: int)
    ensures ring_mul_int(ring_zero_int(), a) == ring_zero_int()
{
    assert(0 * a == 0);
}

} // verus!