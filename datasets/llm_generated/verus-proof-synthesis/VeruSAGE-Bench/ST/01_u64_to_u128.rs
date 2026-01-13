use vstd::prelude::*;

fn main() {}

verus! {

proof fn u64_to_u128_bounded(x: u64)
    ensures
        x as u128 <= u128::MAX,
{
}

}
