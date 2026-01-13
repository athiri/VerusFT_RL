use vstd::prelude::*;

verus! {

pub enum List { Nil, Cons { head: nat, tail: Box<List> } }


pub open spec fn nth(l: List, n: nat) -> Option<nat> decreases l, n {
    match l {
        List::Nil => None,
        List::Cons { head, tail } => if n == 0 { Some(head) } else { nth(*tail, (n-1) as nat) }
    }
}

} // verus!