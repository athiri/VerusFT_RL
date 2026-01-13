use vstd::prelude::*;

verus! {

pub open spec fn seq_forall_dec(s: Seq<nat>, p: spec_fn(nat) -> bool) -> bool
    decreases s.len()
{
    if s.len() == 0 {
        true
    } else {
        p(s[0]) && seq_forall_dec(s.skip(1), p)
    }
}


pub proof fn forall_dec_correct(s: Seq<nat>, p: spec_fn(nat) -> bool)
    ensures seq_forall_dec(s, p) <==> forall|i: int| 0 <= i < s.len() as int ==> p(s[i])
    decreases s.len()
{
    reveal_with_fuel(seq_forall_dec, 2);
    if s.len() > 0 {
        forall_dec_correct(s.skip(1), p);
    }
    assume(seq_forall_dec(s, p) <==> forall|i: int| 0 <= i < s.len() as int ==> p(s[i]));
}

} // verus!