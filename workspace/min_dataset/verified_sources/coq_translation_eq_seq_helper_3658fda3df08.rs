use vstd::prelude::*;

verus! {

pub open spec fn eq_seq_helper<T>(s1: Seq<T>, s2: Seq<T>, eq_t: spec_fn(T, T) -> bool, idx: int) -> bool
    decreases s1.len() - idx
{
    if idx >= s1.len() {
        true
    } else {
        eq_t(s1[idx], s2[idx]) && eq_seq_helper(s1, s2, eq_t, idx + 1)
    }
}

} // verus!