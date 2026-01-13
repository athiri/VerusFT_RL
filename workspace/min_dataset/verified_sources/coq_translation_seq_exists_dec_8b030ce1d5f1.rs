use vstd::prelude::*;

verus! {

pub open spec fn seq_exists_dec(s: Seq<nat>, p: spec_fn(nat) -> bool) -> bool
    decreases s.len()
{
    if s.len() == 0 {
        false
    } else {
        p(s[0]) || seq_exists_dec(s.skip(1), p)
    }
}

} // verus!