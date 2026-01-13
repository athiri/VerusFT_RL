use vstd::prelude::*;

verus! {

pub enum List { Nil, Cons { head: nat, tail: Box<List> } }

pub open spec fn len(l: List) -> nat decreases l {
    match l { List::Nil => 0, List::Cons { tail, .. } => 1 + len(*tail) }
}

pub open spec fn map(l: List, f: spec_fn(nat) -> nat) -> List
    decreases l
{
    match l {
        List::Nil => List::Nil,
        List::Cons { head, tail } => List::Cons { head: f(head), tail: Box::new(map(*tail, f)) }
    }
}



pub proof fn map_len(l: List, f: spec_fn(nat) -> nat)
    ensures len(map(l, f)) == len(l) decreases l
{
    reveal_with_fuel(len, 2); match l { List::Nil => {} List::Cons { tail, .. } => { map_len(*tail, f); } }
}

} // verus!