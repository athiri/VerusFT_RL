use vstd::prelude::*;

verus! {

pub open spec fn remove_at(s: Seq<nat>, i: int) -> Seq<nat>
    recommends 0 <= i < s.len()
{
    s.take(i).add(s.skip(i + 1))
}

pub open spec fn find_min_idx(s: Seq<nat>) -> int
    recommends s.len() > 0
    decreases s.len()
{
    if s.len() <= 1 {
        0
    } else {
        let tail = s.subrange(1, s.len() as int);
        let rest_idx = find_min_idx(tail);
        if rest_idx + 1 < s.len() && s[0] <= s[rest_idx + 1] { 0 } else { rest_idx + 1 }
    }
}


pub open spec fn selection_sort(s: Seq<nat>, fuel: nat) -> Seq<nat>
    decreases fuel
{
    if fuel == 0 || s.len() == 0 {
        s
    } else {
        let min_idx = find_min_idx(s);
        let min_val = s[min_idx];
        let rest = remove_at(s, min_idx);
        seq![min_val].add(selection_sort(rest, (fuel - 1) as nat))
    }
}

} // verus!