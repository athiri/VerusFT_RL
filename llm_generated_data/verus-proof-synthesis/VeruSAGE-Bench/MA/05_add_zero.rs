use vstd::prelude::*;

fn main() {}

verus! {

proof fn add_zero(x: int)
    ensures x + 0 == x,
{
}

}
