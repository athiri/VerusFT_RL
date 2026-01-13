use vstd::prelude::*;

verus! {

pub enum LazyList { Nil, Cons { head: nat, tail: Box<LazyList> } }

pub open spec fn take(l: LazyList, n: nat) -> Seq<nat> decreases n, l {
    if n == 0 { Seq::empty() }
    else { match l { LazyList::Nil => Seq::empty(), LazyList::Cons { head, tail } => seq![head] + take(*tail, (n-1) as nat) } }
}


pub proof fn take_len(l: LazyList, n: nat) ensures take(l, n).len() <= n decreases n, l {
    reveal_with_fuel(take, 2);
    if n > 0 { match l { LazyList::Nil => {} LazyList::Cons { tail, .. } => { take_len(*tail, (n-1) as nat); } } }
}

} // verus!