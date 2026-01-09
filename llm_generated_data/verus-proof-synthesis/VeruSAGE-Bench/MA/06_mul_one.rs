use vstd::prelude::*;

fn main() {}

verus! {

proof fn mul_one(x: int)
    ensures x * 1 == x,
{
}

}
