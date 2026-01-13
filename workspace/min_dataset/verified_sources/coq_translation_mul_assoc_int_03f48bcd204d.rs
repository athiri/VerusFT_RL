use vstd::prelude::*;

verus! {

pub proof fn mul_assoc_int(a: int, b: int, c: int) ensures (a * b) * c == a * (b * c) {
    assert((a * b) * c == a * (b * c)) by (nonlinear_arith);
}

} // verus!