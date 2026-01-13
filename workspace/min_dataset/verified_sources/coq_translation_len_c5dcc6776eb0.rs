use vstd::prelude::*;

verus! {

pub enum List { Nil, Cons { head: nat, tail: Box<List> } }


pub open spec fn len(l: List) -> nat decreases l {
    match l { List::Nil => 0, List::Cons { tail, .. } => 1 + len(*tail) }
}

} // verus!