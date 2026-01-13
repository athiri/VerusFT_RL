use vstd::prelude::*;

verus! {

pub open spec fn seq_reverse<T>(s: Seq<T>) -> Seq<T> {
    Seq::new(s.len(), |i: int| s[s.len() - 1 - i])
}


pub struct Queue<T> {
    pub front: Seq<T>,  // Dequeue from front
    pub back: Seq<T>,   // Enqueue to back (reversed)
}

pub open spec fn queue_normalize<T>(q: Queue<T>) -> Queue<T> {
    if q.front.len() == 0 && q.back.len() > 0 {
        Queue { front: seq_reverse(q.back), back: Seq::empty() }
    } else {
        q
    }
}

pub open spec fn queue_is_empty<T>(q: Queue<T>) -> bool {
    q.front.len() == 0 && q.back.len() == 0
}

pub open spec fn queue_size<T>(q: Queue<T>) -> nat {
    q.front.len() + q.back.len()
}

pub open spec fn queue_dequeue<T>(q: Queue<T>) -> Queue<T> {
    let nq = queue_normalize(q);
    if nq.front.len() == 0 {
        nq
    } else {
        Queue { front: nq.front.skip(1), back: nq.back }
    }
}


pub proof fn dequeue_size<T>(q: Queue<T>)
    requires !queue_is_empty(q)
    ensures queue_size(queue_dequeue(q)) == queue_size(q) - 1
{
    let nq = queue_normalize(q);
    if q.front.len() == 0 {
        // After normalize, front has back reversed
        assert(nq.front.len() == q.back.len());
    }
}

} // verus!