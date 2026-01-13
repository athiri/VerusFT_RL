use vstd::prelude::*;

verus! {

pub open spec fn count_eq(s: Seq<nat>, v: nat) -> nat
    decreases s.len()
{
    if s.len() == 0 {
        0
    } else if s[0] == v {
        1 + count_eq(s.skip(1), v)
    } else {
        count_eq(s.skip(1), v)
    }
}

} // verus!