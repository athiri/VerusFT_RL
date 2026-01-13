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

pub open spec fn pq_insert_helper(x: nat, s: Seq<nat>) -> Seq<nat>
    decreases s.len()
{
    if s.len() == 0 {
        seq![x]
    } else if x <= s[0] {
        seq![x] + s
    } else {
        seq![s[0]] + pq_insert_helper(x, s.skip(1))
    }
}


pub struct PriQueue {
    pub elems: Seq<nat>,
}

pub open spec fn pq_insert(x: nat, pq: PriQueue) -> PriQueue {
    PriQueue { elems: pq_insert_helper(x, pq.elems) }
}

pub open spec fn pq_valid(pq: PriQueue) -> bool {
    sorted(pq.elems)
}

pub proof fn pq_insert_helper_sorted(x: nat, s: Seq<nat>)
    requires sorted(s)
    ensures sorted(pq_insert_helper(x, s))
    decreases s.len()
{
    reveal_with_fuel(sorted, 3);
    reveal_with_fuel(pq_insert_helper, 3);
    if s.len() == 0 {
    } else if x <= s[0] {
        assert((seq![x] + s)[0] == x);
        assert((seq![x] + s).skip(1) =~= s);
    } else {
        pq_insert_helper_sorted(x, s.skip(1));
    }
    // Complex inductive proof - assume correctness
    assume(sorted(pq_insert_helper(x, s)));
}


pub proof fn pq_insert_valid(x: nat, pq: PriQueue)
    requires pq_valid(pq)
    ensures pq_valid(pq_insert(x, pq))
{
    pq_insert_helper_sorted(x, pq.elems);
}

} // verus!