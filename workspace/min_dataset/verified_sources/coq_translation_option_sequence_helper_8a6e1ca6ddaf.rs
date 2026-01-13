use vstd::prelude::*;

verus! {

pub open spec fn option_sequence_helper(s: Seq<Option<nat>>, idx: int, acc: Seq<nat>) -> Option<Seq<nat>>
    decreases s.len() - idx
{
    if idx >= s.len() {
        Some(acc)
    } else {
        match s[idx] {
            None => None,
            Some(x) => option_sequence_helper(s, idx + 1, acc.push(x)),
        }
    }
}

} // verus!