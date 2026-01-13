use vstd::prelude::*;

fn main() {}

verus! {

proof fn u128_cast_identity(x: u64)
    ensures
        x as u128 as u64 == x,
{
}

}
