use vstd::prelude::*;

verus! {

pub open spec fn shrink_nat(n: nat) -> Seq<nat>
    decreases n
{
    if n == 0 {
        seq![]
    } else {
        seq![0nat, n / 2] + if n > 1 { seq![(n - 1) as nat] } else { seq![] }
    }
}

} // verus!