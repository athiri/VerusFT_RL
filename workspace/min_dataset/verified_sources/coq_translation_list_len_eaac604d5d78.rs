use vstd::prelude::*;

verus! {

pub enum List<T> {
    Nil,
    Cons { head: T, tail: Box<List<T>> },
}


pub open spec fn list_len<T>(l: List<T>) -> nat
    decreases l
{
    match l {
        List::Nil => 0,
        List::Cons { head: _, tail } => 1 + list_len(*tail),
    }
}

} // verus!