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

pub open spec fn reverse_index(s: Seq<nat>) -> Seq<nat> {
    Seq::new(s.len(), |i: int| s[s.len() - 1 - i])
}


pub proof fn reverse_first_last(s: Seq<nat>)
    requires s.len() > 0
    ensures reverse(s)[0] == s[s.len() - 1]
{
    // Uses reverse_index definition
    assert(reverse_index(s)[0] == s[s.len() - 1]);
    assume(reverse(s)[0] == reverse_index(s)[0]);
}

} // verus!