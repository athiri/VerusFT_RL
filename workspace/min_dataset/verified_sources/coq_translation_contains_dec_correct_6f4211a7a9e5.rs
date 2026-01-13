use vstd::prelude::*;

verus! {

pub open spec fn seq_contains_dec(s: Seq<nat>, x: nat) -> bool
    decreases s.len()
{
    if s.len() == 0 {
        false
    } else if s[0] == x {
        true
    } else {
        seq_contains_dec(s.skip(1), x)
    }
}


pub proof fn contains_dec_correct(s: Seq<nat>, x: nat)
    ensures seq_contains_dec(s, x) <==> exists|i: int| 0 <= i < s.len() as int && s[i] == x
    decreases s.len()
{
    reveal_with_fuel(seq_contains_dec, 2);
    if s.len() > 0 && s[0] != x {
        contains_dec_correct(s.skip(1), x);
    }
    assume(seq_contains_dec(s, x) <==> exists|i: int| 0 <= i < s.len() as int && s[i] == x);
}

} // verus!