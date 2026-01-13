use vstd::prelude::*;

verus! {

pub type Str = Seq<nat>;


pub open spec fn show_list_helper(s: Seq<nat>, show_elem: spec_fn(nat) -> Str, idx: int) -> Str
    decreases s.len() - idx
{
    if idx >= s.len() {
        Seq::empty()
    } else if idx == s.len() - 1 {
        show_elem(s[idx])
    } else {
        show_elem(s[idx]) + seq![44nat, 32nat] + show_list_helper(s, show_elem, idx + 1)
    }
}

} // verus!