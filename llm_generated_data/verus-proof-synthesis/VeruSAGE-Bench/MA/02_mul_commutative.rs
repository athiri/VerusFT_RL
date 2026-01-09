use vstd::prelude::*;

fn main() {}

verus! {

proof fn mul_commutative(x: nat, y: nat)
    ensures x * y == y * x,
{
}

}
