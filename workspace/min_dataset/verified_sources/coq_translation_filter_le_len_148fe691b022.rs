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


pub proof fn filter_le_len(s: Seq<nat>, pivot: nat)
    ensures filter_le(s, pivot).len() <= s.len()
    decreases s.len()
{
    reveal_with_fuel(filter_le, 2);
    if s.len() > 0 {
        filter_le_len(s.skip(1), pivot);
    }
}

} // verus!