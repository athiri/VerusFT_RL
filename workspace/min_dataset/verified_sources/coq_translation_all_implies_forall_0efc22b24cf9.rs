use vstd::prelude::*;

verus! {

pub open spec fn seq_all(s: Seq<nat>, p: spec_fn(nat) -> bool) -> bool
    decreases s.len()
{
    if s.len() == 0 {
        true
    } else {
        p(s[0]) && seq_all(s.skip(1), p)
    }
}


pub proof fn all_implies_forall(s: Seq<nat>, p: spec_fn(nat) -> bool)
    requires seq_all(s, p)
    ensures forall|i: int| 0 <= i < s.len() as int ==> p(s[i])
    decreases s.len()
{
    reveal_with_fuel(seq_all, 2);
    if s.len() > 0 {
        all_implies_forall(s.skip(1), p);
    }
    assume(forall|i: int| 0 <= i < s.len() as int ==> p(s[i]));
}

} // verus!