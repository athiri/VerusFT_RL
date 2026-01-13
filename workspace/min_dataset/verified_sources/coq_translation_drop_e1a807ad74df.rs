use vstd::prelude::*;

verus! {

pub enum List { Nil, Cons { head: nat, tail: Box<List> } }


pub open spec fn drop(l: List, n: nat) -> List decreases l, n {
    if n == 0 { l }
    else { match l {
        List::Nil => List::Nil,
        List::Cons { tail, .. } => drop(*tail, (n-1) as nat)
    }}
}

} // verus!