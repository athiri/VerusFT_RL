use vstd::prelude::*;

fn main() {}

verus! {

proof fn nat_transitive(a: nat, b: nat, c: nat)
    requires
        a <= b,
        b <= c,
    ensures
        a <= c,
{
}

}
