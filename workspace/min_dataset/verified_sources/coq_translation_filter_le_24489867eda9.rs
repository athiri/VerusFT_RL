use vstd::prelude::*;

verus! {

pub open spec fn filter_le(s: Seq<nat>, pivot: nat) -> Seq<nat>
    decreases s.len()
{
    if s.len() == 0 {
        Seq::empty()
    } else if s[0] <= pivot {
        seq![s[0]] + filter_le(s.skip(1), pivot)
    } else {
        filter_le(s.skip(1), pivot)
    }
}

} // verus!