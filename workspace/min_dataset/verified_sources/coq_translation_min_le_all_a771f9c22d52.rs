use vstd::prelude::*;

verus! {

pub open spec fn find_min(s: Seq<nat>) -> nat
    recommends s.len() > 0
    decreases s.len()
{
    if s.len() <= 1 {
        if s.len() == 0 { 0 } else { s[0] }
    } else {
        let tail = s.subrange(1, s.len() as int);
        let rest_min = find_min(tail);
        if s[0] <= rest_min { s[0] } else { rest_min }
    }
}


pub proof fn min_le_all(s: Seq<nat>, i: int)
    requires s.len() > 0, 0 <= i < s.len()
    ensures find_min(s) <= s[i]
    decreases s.len()
{
    reveal_with_fuel(find_min, 2);
    if s.len() == 1 {
    } else if i == 0 {
    } else {
        min_le_all(s.skip(1), i - 1);
    }
}

} // verus!