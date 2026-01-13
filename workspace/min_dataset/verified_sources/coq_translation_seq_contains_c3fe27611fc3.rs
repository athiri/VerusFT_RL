use vstd::prelude::*;

verus! {

pub open spec fn seq_contains(s: Seq<nat>, x: nat) -> bool
    decreases s.len()
{
    if s.len() == 0 {
        false
    } else {
        s[0] == x || seq_contains(s.skip(1), x)
    }
}

} // verus!