use vstd::prelude::*;

fn main() {}

verus! {

proof fn add_bounded(a: nat, b: nat, c: nat)
    requires
        a <= b,
    ensures
        a + c <= b + c,
{
}

}
