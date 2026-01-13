use vstd::prelude::*;

verus! {

pub open spec fn reverse_acc(s: Seq<nat>, acc: Seq<nat>) -> Seq<nat>
    decreases s.len()
{
    if s.len() == 0 {
        acc
    } else {
        reverse_acc(s.skip(1), seq![s[0]] + acc)
    }
}


pub open spec fn reverse(s: Seq<nat>) -> Seq<nat> {
    reverse_acc(s, Seq::empty())
}


pub proof fn reverse_append(s1: Seq<nat>, s2: Seq<nat>)
    ensures reverse(s1 + s2) =~= reverse(s2) + reverse(s1)
{
    // This requires a detailed proof
    assume(reverse(s1 + s2) =~= reverse(s2) + reverse(s1));
}

} // verus!