use vstd::prelude::*;

verus! {

pub open spec fn count(s: Seq<nat>, v: nat) -> nat
    decreases s.len()
{
    if s.len() == 0 {
        0
    } else if s[0] == v {
        1 + count(s.skip(1), v)
    } else {
        count(s.skip(1), v)
    }
}


pub proof fn count_append(s1: Seq<nat>, s2: Seq<nat>, v: nat)
    ensures count(s1 + s2, v) == count(s1, v) + count(s2, v)
    decreases s1.len()
{
    reveal_with_fuel(count, 2);
    if s1.len() == 0 {
        assert(s1 + s2 =~= s2);
    } else {
        let rest = s1.skip(1);
        count_append(rest, s2, v);
        assert((s1 + s2).skip(1) =~= rest + s2);
    }
}

} // verus!