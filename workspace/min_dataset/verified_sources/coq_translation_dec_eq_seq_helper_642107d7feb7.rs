use vstd::prelude::*;

verus! {

pub enum Dec {
    Yes,
    No,
}


pub open spec fn dec_eq_seq_helper<T>(
    s1: Seq<T>,
    s2: Seq<T>,
    dec_eq_t: spec_fn(T, T) -> Dec,
    i: int
) -> Dec
    decreases s1.len() - i when i >= 0
{
    if i >= s1.len() {
        Dec::Yes
    } else {
        match dec_eq_t(s1[i], s2[i]) {
            Dec::No => Dec::No,
            Dec::Yes => dec_eq_seq_helper(s1, s2, dec_eq_t, i + 1),
        }
    }
}

} // verus!