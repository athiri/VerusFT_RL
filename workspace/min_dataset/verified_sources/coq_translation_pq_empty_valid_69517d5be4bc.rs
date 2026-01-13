use vstd::prelude::*;

verus! {

pub open spec fn sorted(s: Seq<nat>) -> bool
    decreases s.len()
{
    if s.len() <= 1 {
        true
    } else {
        s[0] <= s[1] && sorted(s.skip(1))
    }
}


pub struct PriQueue {
    pub elems: Seq<nat>,
}

pub open spec fn pq_valid(pq: PriQueue) -> bool {
    sorted(pq.elems)
}

pub open spec fn pq_empty() -> PriQueue {
    PriQueue { elems: Seq::empty() }
}


pub proof fn pq_empty_valid()
    ensures pq_valid(pq_empty())
{
}

} // verus!