use vstd::prelude::*;

verus! {

pub open spec fn seq_forall_dec(s: Seq<nat>, p: spec_fn(nat) -> bool) -> bool
    decreases s.len()
{
    if s.len() == 0 {
        true
    } else {
        p(s[0]) && seq_forall_dec(s.skip(1), p)
    }
}

} // verus!