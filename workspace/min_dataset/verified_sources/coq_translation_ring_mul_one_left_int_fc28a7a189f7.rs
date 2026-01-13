use vstd::prelude::*;

verus! {

pub open spec fn ring_mul_int(a: int, b: int) -> int {
    a * b
}

pub open spec fn ring_one_int() -> int {
    1
}


pub proof fn ring_mul_one_left_int(a: int)
    ensures ring_mul_int(ring_one_int(), a) == a
{
    assert(1 * a == a);
}

} // verus!