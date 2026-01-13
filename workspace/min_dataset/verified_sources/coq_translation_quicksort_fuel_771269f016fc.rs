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


pub open spec fn quicksort_fuel(s: Seq<nat>, fuel: nat) -> Seq<nat>
    decreases fuel
{
    if fuel == 0 || s.len() <= 1 {
        s
    } else {
        let pivot = s[0];
        let rest = s.subrange(1, s.len() as int);
        let lo = filter_le(rest, pivot);
        let hi = filter_gt(rest, pivot);
        quicksort_fuel(lo, (fuel - 1) as nat) + seq![pivot] + quicksort_fuel(hi, (fuel - 1) as nat)
    }
}

} // verus!