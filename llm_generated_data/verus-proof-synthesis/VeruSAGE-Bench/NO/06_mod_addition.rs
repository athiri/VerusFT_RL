use vstd::prelude::*;

fn main() {}

verus! {

proof fn mod_addition(x: int, n: int)
    requires
        n > 0,
    ensures
        (x + n) % n == x % n,
{
    assert((x + n) % n == x % n) by(nonlinear_arith);
}

}
