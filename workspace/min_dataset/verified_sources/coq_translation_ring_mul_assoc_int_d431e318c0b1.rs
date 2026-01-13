use vstd::prelude::*;

verus! {

pub open spec fn ring_mul_int(a: int, b: int) -> int {
    a * b
}


pub proof fn ring_mul_assoc_int(a: int, b: int, c: int)
    ensures ring_mul_int(ring_mul_int(a, b), c) == ring_mul_int(a, ring_mul_int(b, c))
{
    assert((a * b) * c == a * (b * c)) by(nonlinear_arith);
}

} // verus!