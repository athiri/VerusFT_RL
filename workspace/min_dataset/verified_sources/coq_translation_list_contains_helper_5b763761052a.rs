use vstd::prelude::*;

verus! {

pub open spec fn list_contains_helper<T>(s: Seq<T>, x: T, eq: spec_fn(T, T) -> bool, i: int) -> bool
    decreases s.len() - i when i >= 0
{
    if i >= s.len() {
        false
    } else if eq(s[i], x) {
        true
    } else {
        list_contains_helper(s, x, eq, i + 1)
    }
}

} // verus!