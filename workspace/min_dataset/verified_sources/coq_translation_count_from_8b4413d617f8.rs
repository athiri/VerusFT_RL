use vstd::prelude::*;

verus! {

pub open spec fn count_from(s: Seq<nat>, v: nat, start: nat) -> nat
    decreases s.len() - start
{
    if start >= s.len() {
        0
    } else if s[start as int] == v {
        1 + count_from(s, v, start + 1)
    } else {
        count_from(s, v, start + 1)
    }
}

} // verus!