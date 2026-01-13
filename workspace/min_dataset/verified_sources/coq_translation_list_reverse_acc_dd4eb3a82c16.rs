use vstd::prelude::*;

verus! {

pub enum List<T> {
    Nil,
    Cons { head: T, tail: Box<List<T>> },
}


pub open spec fn list_reverse_acc<T>(l: List<T>, acc: List<T>) -> List<T>
    decreases l
{
    match l {
        List::Nil => acc,
        List::Cons { head, tail } => list_reverse_acc(*tail, List::Cons { head, tail: Box::new(acc) }),
    }
}

} // verus!