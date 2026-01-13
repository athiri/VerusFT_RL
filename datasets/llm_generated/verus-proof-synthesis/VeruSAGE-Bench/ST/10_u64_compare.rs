use vstd::prelude::*;

fn main() {}

verus! {

proof fn u64_compare(a: u64, b: u64)
    requires
        a <= b,
    ensures
        a as u128 <= b as u128,
{
}

}
