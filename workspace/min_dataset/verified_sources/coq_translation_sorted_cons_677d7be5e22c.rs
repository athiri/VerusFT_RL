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


pub proof fn sorted_cons(x: nat, s: Seq<nat>)
    requires
        sorted(s),
        s.len() == 0 || x <= s[0]
    ensures sorted(seq![x].add(s))
    decreases s.len()
{
    reveal_with_fuel(sorted, 3);
    if s.len() == 0 {
        // seq![x] is sorted
    } else {
        // x <= s[0] and sorted(s), so seq![x] + s is sorted
        assert(seq![x].add(s).skip(1) =~= s);
    }
}

} // verus!