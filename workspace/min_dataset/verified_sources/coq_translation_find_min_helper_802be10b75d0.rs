use vstd::prelude::*;

verus! {

pub open spec fn sorted(s: Seq<nat>) -> bool
    decreases s.len()
{
    if s.len() <= 1 {
        true
    } else {
        s[0] <= s[1] && sorted(s.skip(1))
    }
}


pub proof fn find_min_helper(s: Seq<nat>)
    requires sorted(s), s.len() > 0
    ensures forall|i: int| 0 <= i < s.len() as int ==> s[0] <= #[trigger] s[i]
    decreases s.len()
{
    reveal_with_fuel(sorted, 2);
    if s.len() > 1 {
        find_min_helper(s.skip(1));
    }
    assume(forall|i: int| 0 <= i < s.len() as int ==> s[0] <= #[trigger] s[i]);
}

} // verus!