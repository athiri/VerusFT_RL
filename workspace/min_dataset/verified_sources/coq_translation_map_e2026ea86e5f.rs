use vstd::prelude::*;

verus! {

pub enum List { Nil, Cons { head: nat, tail: Box<List> } }


pub open spec fn map(l: List, f: spec_fn(nat) -> nat) -> List decreases l {
    match l {
        List::Nil => List::Nil,
        List::Cons { head, tail } => List::Cons { head: f(head), tail: Box::new(map(*tail, f)) }
    }
}

} // verus!