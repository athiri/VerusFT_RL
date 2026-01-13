use vstd::prelude::*;

verus! {

pub open spec fn seq_sum_rec(s: Seq<nat>) -> nat
    decreases s.len()
{
    if s.len() == 0 {
        0
    } else {
        s[0] + seq_sum_rec(s.skip(1))
    }
}

} // verus!