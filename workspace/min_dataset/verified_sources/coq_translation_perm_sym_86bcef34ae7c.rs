use vstd::prelude::*;

verus! {

pub open spec fn count(s: Seq<nat>, x: nat) -> nat decreases s.len() {
    if s.len() == 0 { 0 }
    else if s[0] == x { 1 + count(s.skip(1), x) }
    else { count(s.skip(1), x) }
}


pub open spec fn is_permutation(s1: Seq<nat>, s2: Seq<nat>) -> bool {
    s1.len() == s2.len() && forall|x: nat| count(s1, x) == count(s2, x)
}


pub proof fn perm_sym(s1: Seq<nat>, s2: Seq<nat>)
    requires is_permutation(s1, s2)
    ensures is_permutation(s2, s1)
{}

} // verus!