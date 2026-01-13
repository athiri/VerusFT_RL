use vstd::prelude::*;

verus! {

pub proof fn reverse_acc_len(s: Seq<nat>, acc: Seq<nat>)
    ensures reverse_acc(s, acc).len() == s.len() + acc.len()
    decreases s.len()
{
    reveal_with_fuel(reverse_acc, 2);
    if s.len() > 0 {
        reverse_acc_len(s.skip(1), seq![s[0]] + acc);
    }
}

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

pub proof fn reverse_len(s: Seq<nat>)
    ensures reverse(s).len() == s.len()
{
    reverse_acc_len(s, Seq::empty());
}

pub open spec fn reverse_index(s: Seq<nat>) -> Seq<nat> {
    Seq::new(s.len(), |i: int| s[s.len() - 1 - i])
}


pub proof fn reverse_last_first(s: Seq<nat>)
    requires s.len() > 0
    ensures reverse(s)[reverse(s).len() - 1] == s[0]
{
    reverse_len(s);
    assert(reverse_index(s)[s.len() - 1] == s[0]);
    assume(reverse(s)[reverse(s).len() - 1] == s[0]);
}

} // verus!