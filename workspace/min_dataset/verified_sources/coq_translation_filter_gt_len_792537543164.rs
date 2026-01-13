use vstd::prelude::*;

verus! {

pub open spec fn filter_gt(s: Seq<nat>, pivot: nat) -> Seq<nat>
    decreases s.len()
{
    if s.len() == 0 {
        Seq::empty()
    } else if s[0] > pivot {
        seq![s[0]] + filter_gt(s.skip(1), pivot)
    } else {
        filter_gt(s.skip(1), pivot)
    }
}


pub proof fn filter_gt_len(s: Seq<nat>, pivot: nat)
    ensures filter_gt(s, pivot).len() <= s.len()
    decreases s.len()
{
    reveal_with_fuel(filter_gt, 2);
    if s.len() > 0 {
        filter_gt_len(s.skip(1), pivot);
    }
}

} // verus!