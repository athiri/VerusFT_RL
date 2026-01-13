use vstd::prelude::*;

verus! {

pub open spec fn seq_count(s: Seq<nat>, p: spec_fn(nat) -> bool) -> nat
    decreases s.len()
{
    if s.len() == 0 {
        0
    } else {
        (if p(s[0]) { 1nat } else { 0nat }) + seq_count(s.skip(1), p)
    }
}

} // verus!