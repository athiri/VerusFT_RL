use vstd::prelude::*;

verus! {

pub open spec fn seq_min(s: Seq<nat>, default: nat) -> nat
    decreases s.len()
{
    if s.len() == 0 {
        default
    } else if s.len() == 1 {
        s[0]
    } else {
        let rest_min = seq_min(s.skip(1), default);
        if s[0] <= rest_min { s[0] } else { rest_min }
    }
}

} // verus!