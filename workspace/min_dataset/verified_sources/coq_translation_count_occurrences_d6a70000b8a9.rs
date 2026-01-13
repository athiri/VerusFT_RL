use vstd::prelude::*;

verus! {

pub open spec fn count_occurrences(s: Seq<nat>, x: nat) -> nat
    decreases s.len()
{
    if s.len() == 0 {
        0
    } else if s[0] == x {
        1 + count_occurrences(s.skip(1), x)
    } else {
        count_occurrences(s.skip(1), x)
    }
}

} // verus!