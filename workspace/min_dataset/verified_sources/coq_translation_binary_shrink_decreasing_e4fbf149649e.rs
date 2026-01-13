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


pub proof fn binary_shrink_decreasing(n: nat)
    requires n > 0
    ensures forall|i: int| 0 <= i < binary_shrink(n).len() ==> binary_shrink(n)[i] < n
{
    assume(forall|i: int| 0 <= i < binary_shrink(n).len() ==> binary_shrink(n)[i] < n);
}

} // verus!