use vstd::prelude::*;

verus! {

pub open spec fn seq_max(s: Seq<nat>, default: nat) -> nat
    decreases s.len()
{
    if s.len() == 0 {
        default
    } else if s.len() == 1 {
        s[0]
    } else {
        let rest_max = seq_max(s.skip(1), default);
        if s[0] >= rest_max { s[0] } else { rest_max }
    }
}

} // verus!