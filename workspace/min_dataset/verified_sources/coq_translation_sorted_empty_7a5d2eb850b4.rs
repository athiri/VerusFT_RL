use vstd::prelude::*;

verus! {

pub open spec fn sorted(s: Seq<nat>) -> bool decreases s.len() {
    s.len() <= 1 || (s[0] <= s[1] && sorted(s.skip(1)))
}


pub proof fn sorted_empty() ensures sorted(Seq::<nat>::empty()) {}

} // verus!