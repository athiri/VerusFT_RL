use vstd::prelude::*;

verus! {

pub struct PriQueue {
    pub elems: Seq<nat>,
}

pub open spec fn pq_delete_min(pq: PriQueue) -> PriQueue {
    if pq.elems.len() == 0 {
        pq
    } else {
        PriQueue { elems: pq.elems.skip(1) }
    }
}

pub open spec fn sorted(s: Seq<nat>) -> bool
    decreases s.len()
{
    if s.len() <= 1 {
        true
    } else {
        s[0] <= s[1] && sorted(s.skip(1))
    }
}

pub open spec fn pq_valid(pq: PriQueue) -> bool {
    sorted(pq.elems)
}


pub proof fn pq_delete_min_valid(pq: PriQueue)
    requires pq_valid(pq)
    ensures pq_valid(pq_delete_min(pq))
{
    reveal_with_fuel(sorted, 2);
}

} // verus!