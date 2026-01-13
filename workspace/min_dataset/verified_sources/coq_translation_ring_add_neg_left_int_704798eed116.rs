use vstd::prelude::*;

verus! {

pub open spec fn ring_zero_int() -> int {
    0
}

pub open spec fn ring_neg_int(a: int) -> int {
    -a
}

pub open spec fn ring_add_int(a: int, b: int) -> int {
    a + b
}


pub proof fn ring_add_neg_left_int(a: int)
    ensures ring_add_int(ring_neg_int(a), a) == ring_zero_int()
{
    assert(-a + a == 0);
}

} // verus!