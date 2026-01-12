use vstd::prelude::*;

fn main() {}

verus! {

proof fn sub_self(x: int)
    ensures x - x == 0,
{
}

}
