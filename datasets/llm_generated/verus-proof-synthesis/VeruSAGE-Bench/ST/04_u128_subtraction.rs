use vstd::prelude::*;

fn main() {}

verus! {

proof fn u128_subtraction(a: u128, b: u128)
    requires
        a >= b,
    ensures
        a - b <= a,
{
}

}
