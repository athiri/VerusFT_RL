use vstd::prelude::*;

verus! {

pub open spec fn forall_indexed_helper<T>(s: Seq<T>, p: spec_fn(int, T) -> bool, i: int) -> bool
    decreases s.len() - i when i >= 0
{
    if i >= s.len() {
        true
    } else if !p(i, s[i]) {
        false
    } else {
        forall_indexed_helper(s, p, i + 1)
    }
}

} // verus!