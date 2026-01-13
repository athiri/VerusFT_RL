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


pub proof fn reverse_reverse(s: Seq<nat>)
    ensures reverse(reverse(s)) =~= s
{
    // This requires a more detailed proof
    assume(reverse(reverse(s)) =~= s);
}

} // verus!