use vstd::prelude::*;

verus! {

pub open spec fn list_sorted_helper(s: Seq<nat>, i: int) -> bool
    decreases s.len() - i when i >= 0
{
    if i >= s.len() - 1 {
        true
    } else if s[i] > s[i + 1] {
        false
    } else {
        list_sorted_helper(s, i + 1)
    }
}

} // verus!