use vstd::prelude::*;

verus! {

pub open spec fn ring_mul_int(a: int, b: int) -> int {
    a * b
}

pub open spec fn ring_add_int(a: int, b: int) -> int {
    a + b
}


pub proof fn ring_distrib_left_int(a: int, b: int, c: int)
    ensures ring_mul_int(a, ring_add_int(b, c)) == ring_add_int(ring_mul_int(a, b), ring_mul_int(a, c))
{
    assert(a * (b + c) == a * b + a * c) by(nonlinear_arith);
}

} // verus!