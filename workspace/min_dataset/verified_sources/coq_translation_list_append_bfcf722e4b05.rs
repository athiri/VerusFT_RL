use vstd::prelude::*;

verus! {

pub enum List<T> {
    Nil,
    Cons { head: T, tail: Box<List<T>> },
}


pub open spec fn list_append<T>(l1: List<T>, l2: List<T>) -> List<T>
    decreases l1
{
    match l1 {
        List::Nil => l2,
        List::Cons { head, tail } => List::Cons { head, tail: Box::new(list_append(*tail, l2)) },
    }
}

} // verus!