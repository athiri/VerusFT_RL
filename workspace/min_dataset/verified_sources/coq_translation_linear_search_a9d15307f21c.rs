use vstd::prelude::*;

verus! {

pub open spec fn linear_search(s: Seq<nat>, target: nat, i: nat) -> Option<nat>
    decreases s.len() - i
{
    if i >= s.len() { None }
    else if s[i as int] == target { Some(i) }
    else { linear_search(s, target, i + 1) }
}

} // verus!