use vstd::prelude::*;

verus! {

pub open spec fn ring_zero_int() -> int {
    0
}

pub open spec fn ring_add_int(a: int, b: int) -> int {
    a + b
}


pub proof fn ring_add_zero_right_int(a: int)
    ensures ring_add_int(a, ring_zero_int()) == a
{
    assert(a + 0 == a);
}

} // verus!