use vstd::prelude::*;

verus! {

pub open spec fn linear_shrink(n: nat) -> Seq<nat>
    decreases n
{
    if n == 0 {
        seq![]
    } else {
        seq![(n - 1) as nat] + linear_shrink((n - 1) as nat)
    }
}

} // verus!