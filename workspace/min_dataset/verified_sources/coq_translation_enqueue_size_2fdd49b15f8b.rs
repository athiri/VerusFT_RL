use vstd::prelude::*;

verus! {

pub struct Queue<T> {
    pub front: Seq<T>,  // Dequeue from front
    pub back: Seq<T>,   // Enqueue to back (reversed)
}

pub open spec fn queue_size<T>(q: Queue<T>) -> nat {
    q.front.len() + q.back.len()
}

pub open spec fn queue_enqueue<T>(x: T, q: Queue<T>) -> Queue<T> {
    Queue { front: q.front, back: q.back.push(x) }
}


pub proof fn enqueue_size<T>(x: T, q: Queue<T>)
    ensures queue_size(queue_enqueue(x, q)) == queue_size(q) + 1
{
}

} // verus!