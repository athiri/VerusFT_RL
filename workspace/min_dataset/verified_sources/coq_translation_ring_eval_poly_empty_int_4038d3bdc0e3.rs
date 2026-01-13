use vstd::prelude::*;

verus! {

pub open spec fn ring_mul_int(a: int, b: int) -> int {
    a * b
}

pub open spec fn ring_add_int(a: int, b: int) -> int {
    a + b
}


pub open spec fn ring_eval_poly_int(coeffs: Seq<int>, x: int) -> int
    decreases coeffs.len()
{
    if coeffs.len() == 0 {
        ring_zero_int()
    } else {
        ring_add_int(
            coeffs[0],
            ring_mul_int(x, ring_eval_poly_int(coeffs.skip(1), x))
        )
    }
}

pub open spec fn ring_zero_int() -> int {
    0
}


pub proof fn ring_eval_poly_empty_int(x: int)
    ensures ring_eval_poly_int(Seq::empty(), x) == ring_zero_int()
{
    // Trivially true
}

} // verus!