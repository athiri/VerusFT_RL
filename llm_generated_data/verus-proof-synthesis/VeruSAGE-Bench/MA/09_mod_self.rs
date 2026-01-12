use vstd::prelude::*;

fn main() {}

verus! {

proof fn mod_self(x: int)
    requires x > 0,
    ensures x % x == 0,
{
}

}
