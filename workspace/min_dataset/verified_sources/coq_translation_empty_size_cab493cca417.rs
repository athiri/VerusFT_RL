use vstd::prelude::*;

verus! {

pub struct Queue<T> {
    pub front: Seq<T>,  // Dequeue from front
    pub back: Seq<T>,   // Enqueue to back (reversed)
}

pub open spec fn queue_size<T>(q: Queue<T>) -> nat {
    q.front.len() + q.back.len()
}

pub open spec fn queue_empty<T>() -> Queue<T> {
    Queue { front: Seq::empty(), back: Seq::empty() }
}


pub proof fn empty_size<T>()
    ensures queue_size(queue_empty::<T>()) == 0
{
}

} // verus!