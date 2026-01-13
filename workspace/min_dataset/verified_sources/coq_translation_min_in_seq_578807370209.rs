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


pub proof fn min_in_seq(s: Seq<nat>)
    requires s.len() > 0
    ensures s.contains(find_min(s))
    decreases s.len()
{
    reveal_with_fuel(find_min, 2);
    if s.len() == 1 {
    } else {
        min_in_seq(s.skip(1));
    }
}

} // verus!