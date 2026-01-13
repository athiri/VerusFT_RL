use vstd::prelude::*;

verus! {

pub enum LazyList { Nil, Cons { head: nat, tail: Box<LazyList> } }


pub open spec fn take(l: LazyList, n: nat) -> Seq<nat> decreases n, l {
    if n == 0 { Seq::empty() }
    else { match l { LazyList::Nil => Seq::empty(), LazyList::Cons { head, tail } => seq![head] + take(*tail, (n-1) as nat) } }
}

} // verus!