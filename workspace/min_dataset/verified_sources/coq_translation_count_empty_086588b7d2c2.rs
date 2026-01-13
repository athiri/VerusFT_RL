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


pub proof fn count_empty(v: nat)
    ensures count(Seq::<nat>::empty(), v) == 0
{
}

} // verus!