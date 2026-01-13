use vstd::prelude::*;

verus! {

pub open spec fn seq_any(s: Seq<nat>, p: spec_fn(nat) -> bool) -> bool
    decreases s.len()
{
    if s.len() == 0 {
        false
    } else {
        p(s[0]) || seq_any(s.skip(1), p)
    }
}


pub proof fn any_implies_exists(s: Seq<nat>, p: spec_fn(nat) -> bool)
    requires seq_any(s, p)
    ensures exists|i: int| 0 <= i < s.len() as int && p(s[i])
    decreases s.len()
{
    reveal_with_fuel(seq_any, 2);
    if s.len() > 0 {
        if p(s[0]) {
            assert(0 <= 0 < s.len() as int && p(s[0]));
        } else {
            any_implies_exists(s.skip(1), p);
        }
    }
}

} // verus!