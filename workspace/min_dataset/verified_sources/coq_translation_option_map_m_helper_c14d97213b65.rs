use vstd::prelude::*;

verus! {

pub open spec fn option_map_m_helper(s: Seq<nat>, f: spec_fn(nat) -> Option<nat>, idx: int, acc: Seq<nat>) -> Option<Seq<nat>>
    decreases s.len() - idx
{
    if idx >= s.len() {
        Some(acc)
    } else {
        match f(s[idx]) {
            None => None,
            Some(y) => option_map_m_helper(s, f, idx + 1, acc.push(y)),
        }
    }
}

} // verus!