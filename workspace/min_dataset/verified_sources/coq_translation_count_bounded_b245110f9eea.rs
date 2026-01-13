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


pub proof fn count_bounded(s: Seq<nat>, p: spec_fn(nat) -> bool)
    ensures seq_count(s, p) <= s.len()
    decreases s.len()
{
    reveal_with_fuel(seq_count, 2);
    if s.len() > 0 {
        count_bounded(s.skip(1), p);
    }
}

} // verus!