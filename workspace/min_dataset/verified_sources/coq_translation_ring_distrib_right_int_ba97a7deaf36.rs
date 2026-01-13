use vstd::prelude::*;

verus! {

pub open spec fn ring_mul_int(a: int, b: int) -> int {
    a * b
}

pub open spec fn ring_add_int(a: int, b: int) -> int {
    a + b
}


pub proof fn ring_distrib_right_int(a: int, b: int, c: int)
    ensures ring_mul_int(ring_add_int(a, b), c) == ring_add_int(ring_mul_int(a, c), ring_mul_int(b, c))
{
    assert((a + b) * c == a * c + b * c) by(nonlinear_arith);
}

} // verus!