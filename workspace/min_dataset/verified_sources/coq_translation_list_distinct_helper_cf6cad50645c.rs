use vstd::prelude::*;

verus! {

pub open spec fn list_distinct_helper(s: Seq<nat>, i: int, j: int) -> bool
    decreases s.len() - i, s.len() - j when i >= 0 && j >= 0
{
    if i >= s.len() {
        true
    } else if j >= s.len() {
        list_distinct_helper(s, i + 1, i + 2)
    } else if s[i] == s[j] {
        false
    } else {
        list_distinct_helper(s, i, j + 1)
    }
}

} // verus!