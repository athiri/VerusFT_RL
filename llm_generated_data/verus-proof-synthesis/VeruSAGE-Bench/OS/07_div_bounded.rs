use vstd::prelude::*;

fn main() {}

verus! {

proof fn zero_div(c: nat)
    requires
        c > 0,
    ensures
        0nat / c == 0,
{
}

}
