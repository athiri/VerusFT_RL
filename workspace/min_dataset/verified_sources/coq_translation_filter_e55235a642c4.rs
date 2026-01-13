use vstd::prelude::*;

verus! {

pub enum List { Nil, Cons { head: nat, tail: Box<List> } }


pub open spec fn filter(l: List, p: spec_fn(nat) -> bool) -> List decreases l {
    match l {
        List::Nil => List::Nil,
        List::Cons { head, tail } =>
            if p(head) { List::Cons { head, tail: Box::new(filter(*tail, p)) } }
            else { filter(*tail, p) }
    }
}

} // verus!