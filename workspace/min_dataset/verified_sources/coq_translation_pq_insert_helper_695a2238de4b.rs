use vstd::prelude::*;

verus! {

pub open spec fn pq_insert_helper(x: nat, s: Seq<nat>) -> Seq<nat>
    decreases s.len()
{
    if s.len() == 0 {
        seq![x]
    } else if x <= s[0] {
        seq![x] + s
    } else {
        seq![s[0]] + pq_insert_helper(x, s.skip(1))
    }
}

} // verus!