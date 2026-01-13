use vstd::prelude::*;

verus! {

pub open spec fn enum_all_nat_range(n: nat) -> Seq<nat>
    decreases n
{
    if n == 0 {
        Seq::empty()
    } else {
        enum_all_nat_range((n - 1) as nat).push((n - 1) as nat)
    }
}

} // verus!