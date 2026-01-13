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


pub proof fn reverse_acc_len(s: Seq<nat>, acc: Seq<nat>)
    ensures reverse_acc(s, acc).len() == s.len() + acc.len()
    decreases s.len()
{
    reveal_with_fuel(reverse_acc, 2);
    if s.len() > 0 {
        reverse_acc_len(s.skip(1), seq![s[0]] + acc);
    }
}

} // verus!