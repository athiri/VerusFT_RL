use vstd::prelude::*;

verus! {

pub open spec fn exists_indexed_helper<T>(s: Seq<T>, p: spec_fn(int, T) -> bool, i: int) -> bool
    decreases s.len() - i when i >= 0
{
    if i >= s.len() {
        false
    } else if p(i, s[i]) {
        true
    } else {
        exists_indexed_helper(s, p, i + 1)
    }
}

} // verus!