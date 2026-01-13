use vstd::prelude::*;

verus! {

pub open spec fn seq_any(s: Seq<nat>, p: spec_fn(nat) -> bool) -> bool
    decreases s.len()
{
    if s.len() == 0 {
        false
    } else {
        p(s[0]) || seq_any(s.skip(1), p)
    }
}

} // verus!