use vstd::prelude::*;

fn main() {}

verus! {

proof fn div_by_one(x: int)
    ensures x / 1 == x,
{
}

}
