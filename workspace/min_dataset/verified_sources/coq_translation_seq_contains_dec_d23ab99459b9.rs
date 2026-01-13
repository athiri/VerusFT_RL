use vstd::prelude::*;

verus! {

pub open spec fn seq_contains_dec(s: Seq<nat>, x: nat) -> bool
    decreases s.len()
{
    if s.len() == 0 {
        false
    } else if s[0] == x {
        true
    } else {
        seq_contains_dec(s.skip(1), x)
    }
}

} // verus!