use vstd::prelude::*;

fn main() {}

verus! {

proof fn add_associative(x: int, y: int, z: int)
    ensures (x + y) + z == x + (y + z),
{
}

}
