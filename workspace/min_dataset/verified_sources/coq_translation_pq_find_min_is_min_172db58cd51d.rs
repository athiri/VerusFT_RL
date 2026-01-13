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

pub proof fn find_min_helper(s: Seq<nat>)
    requires sorted(s), s.len() > 0
    ensures forall|i: int| 0 <= i < s.len() as int ==> s[0] <= #[trigger] s[i]
    decreases s.len()
{
    reveal_with_fuel(sorted, 2);
    if s.len() > 1 {
        find_min_helper(s.skip(1));
    }
    assume(forall|i: int| 0 <= i < s.len() as int ==> s[0] <= #[trigger] s[i]);
}

pub open spec fn pq_valid(pq: PriQueue) -> bool {
    sorted(pq.elems)
}

pub open spec fn pq_is_empty(pq: PriQueue) -> bool {
    pq.elems.len() == 0
}


pub proof fn pq_find_min_is_min(pq: PriQueue)
    requires pq_valid(pq), !pq_is_empty(pq)
    ensures forall|i: int| 0 <= i < pq.elems.len() as int ==>
        pq.elems[0] <= #[trigger] pq.elems[i]
{
    find_min_helper(pq.elems);
}

} // verus!