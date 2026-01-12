use vstd::prelude::*;

fn main() {}

verus! {

proof fn mul_zero(a: nat)
    ensures
        a * 0 == 0,
{
}

}
