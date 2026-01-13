use vstd::prelude::*;

verus! {

pub open spec fn count(s: Seq<nat>, v: nat) -> nat
    decreases s.len()
{
    if s.len() == 0 {
        0
    } else if s[0] == v {
        1 + count(s.skip(1), v)
    } else {
        count(s.skip(1), v)
    }
}


pub proof fn count_bounded(s: Seq<nat>, v: nat)
    ensures count(s, v) <= s.len()
    decreases s.len()
{
    reveal_with_fuel(count, 2);
    if s.len() > 0 {
        count_bounded(s.skip(1), v);
    }
}

} // verus!