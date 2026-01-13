use vstd::prelude::*;

verus! {

pub open spec fn reverse(s: Seq<nat>) -> Seq<nat> {
    reverse_acc(s, Seq::empty())
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


pub proof fn reverse_singleton(x: nat)
    ensures reverse(seq![x]) =~= seq![x]
{
    reveal_with_fuel(reverse_acc, 3);
}

} // verus!