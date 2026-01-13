use vstd::prelude::*;

verus! {

pub open spec fn forall_seq_helper<T>(s: Seq<T>, p: spec_fn(T) -> bool, i: int) -> bool
    decreases s.len() - i when i >= 0
{
    if i >= s.len() {
        true
    } else if !p(s[i]) {
        false
    } else {
        forall_seq_helper(s, p, i + 1)
    }
}

} // verus!