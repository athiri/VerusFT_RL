use vstd::prelude::*;

verus! {

pub open spec fn bounded_range(lo: nat, hi: nat) -> Seq<nat>
    recommends lo <= hi
    decreases hi - lo + 1
{
    if lo > hi {
        Seq::empty()
    } else if lo == hi {
        seq![lo]
    } else {
        seq![lo].add(bounded_range((lo + 1) as nat, hi))
    }
}

} // verus!