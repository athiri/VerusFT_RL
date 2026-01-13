use vstd::prelude::*;

fn main() {}

verus! {

proof fn mul_distribute(a: int, b: int, c: int)
    ensures a * (b + c) == a * b + a * c,
{
    assert(a * (b + c) == a * b + a * c) by(nonlinear_arith);
}

}
