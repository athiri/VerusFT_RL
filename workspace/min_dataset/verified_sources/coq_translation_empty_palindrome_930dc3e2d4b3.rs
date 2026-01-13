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


pub proof fn reverse_empty()
    ensures reverse(Seq::<nat>::empty()) =~= Seq::<nat>::empty()
{
    reveal_with_fuel(reverse_acc, 2);
}

pub open spec fn is_palindrome(s: Seq<nat>) -> bool {
    s =~= reverse(s)
}


pub proof fn empty_palindrome()
    ensures is_palindrome(Seq::<nat>::empty())
{
    reverse_empty();
}

} // verus!