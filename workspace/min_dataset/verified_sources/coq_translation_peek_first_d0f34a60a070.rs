use vstd::prelude::*;

verus! {

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

pub open spec fn queue_peek<T>(q: Queue<T>) -> Option<T> {
    let nq = queue_normalize(q);
    if nq.front.len() == 0 {
        None
    } else {
        Some(nq.front[0])
    }
}

pub open spec fn queue_is_empty<T>(q: Queue<T>) -> bool {
    q.front.len() == 0 && q.back.len() == 0
}

pub open spec fn queue_to_seq<T>(q: Queue<T>) -> Seq<T> {
    q.front + seq_reverse(q.back)
}

pub open spec fn seq_reverse<T>(s: Seq<T>) -> Seq<T> {
    Seq::new(s.len(), |i: int| s[s.len() - 1 - i])
}


pub proof fn peek_first<T>(q: Queue<T>)
    requires !queue_is_empty(q)
    ensures queue_peek(q) == Some(queue_to_seq(q)[0])
{
    let nq = queue_normalize(q);
    if q.front.len() == 0 {
        // front was empty, now has reversed back
        assert(nq.front =~= seq_reverse(q.back));
        assert(nq.front[0] == q.back[q.back.len() - 1]);
    }
}

} // verus!