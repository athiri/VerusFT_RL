use vstd::prelude::*;

verus! {

pub open spec fn list_any_helper<T>(s: Seq<T>, p: spec_fn(T) -> bool, i: int) -> bool
    decreases s.len() - i when i >= 0
{
    if i >= s.len() {
        false
    } else if p(s[i]) {
        true
    } else {
        list_any_helper(s, p, i + 1)
    }
}

} // verus!