use vstd::prelude::*;

verus! {

pub proof fn distr_int(a: int, b: int, c: int) ensures a * (b + c) == a * b + a * c {
    assert(a * (b + c) == a * b + a * c) by (nonlinear_arith);
}

} // verus!