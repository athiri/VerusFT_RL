use vstd::prelude::*;

fn main() {}

verus! {

proof fn u64_addition_bounded(a: u64, b: u64)
    requires
        a as u128 + b as u128 <= u64::MAX as u128,
    ensures
        (a + b) as u128 == a as u128 + b as u128,
{
}

}
