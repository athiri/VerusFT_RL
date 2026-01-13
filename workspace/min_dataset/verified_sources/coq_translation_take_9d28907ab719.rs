use vstd::prelude::*;

verus! {

pub enum List { Nil, Cons { head: nat, tail: Box<List> } }


pub open spec fn take(l: List, n: nat) -> List decreases l, n {
    if n == 0 { List::Nil }
    else { match l {
        List::Nil => List::Nil,
        List::Cons { head, tail } => List::Cons { head, tail: Box::new(take(*tail, (n-1) as nat)) }
    }}
}

} // verus!