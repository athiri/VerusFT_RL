use vstd::prelude::*;

verus! {

pub open spec fn ring_mul_int(a: int, b: int) -> int {
    a * b
}

pub open spec fn ring_neg_int(a: int) -> int {
    -a
}


pub proof fn ring_neg_mul_left_int(a: int, b: int)
    ensures ring_mul_int(ring_neg_int(a), b) == ring_neg_int(ring_mul_int(a, b))
{
    assert((-a) * b == -(a * b)) by(nonlinear_arith);
}

} // verus!