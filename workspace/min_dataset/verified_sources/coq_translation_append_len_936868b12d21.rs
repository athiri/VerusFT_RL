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

pub open spec fn list_len<T>(l: List<T>) -> nat
    decreases l
{
    match l {
        List::Nil => 0,
        List::Cons { head: _, tail } => 1 + list_len(*tail),
    }
}


pub proof fn append_len<T>(l1: List<T>, l2: List<T>)
    ensures list_len(list_append(l1, l2)) == list_len(l1) + list_len(l2)
    decreases l1
{
    reveal_with_fuel(list_len, 2);
    reveal_with_fuel(list_append, 2);
    match l1 {
        List::Nil => {}
        List::Cons { head: _, tail } => { append_len(*tail, l2); }
    }
}

} // verus!