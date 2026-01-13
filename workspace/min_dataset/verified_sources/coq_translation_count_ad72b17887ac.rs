use vstd::prelude::*;

verus! {

pub open spec fn count(s: Seq<nat>, x: nat) -> nat decreases s.len() {
    if s.len() == 0 { 0 }
    else if s[0] == x { 1 + count(s.skip(1), x) }
    else { count(s.skip(1), x) }
}

} // verus!