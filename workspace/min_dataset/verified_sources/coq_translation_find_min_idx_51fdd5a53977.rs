use vstd::prelude::*;

verus! {

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

} // verus!