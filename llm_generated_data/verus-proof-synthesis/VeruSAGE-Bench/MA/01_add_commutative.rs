use vstd::prelude::*;

fn main() {}

verus! {

proof fn add_commutative(x: int, y: int)
    ensures x + y == y + x,
{
}

}
