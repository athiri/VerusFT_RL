use vstd::prelude::*;

verus! {

pub open spec fn binary_shrink(n: nat) -> Seq<nat>
    decreases n
{
    if n == 0 {
        seq![]
    } else {
        seq![n / 2] + binary_shrink(n / 2)
    }
}

} // verus!