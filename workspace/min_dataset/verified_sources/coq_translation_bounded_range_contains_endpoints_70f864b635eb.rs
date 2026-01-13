use vstd::prelude::*;

verus! {

pub open spec fn bounded_range(lo: nat, hi: nat) -> Seq<nat>
    recommends lo <= hi
    decreases hi - lo + 1
{
    if lo > hi {
        Seq::empty()
    } else if lo == hi {
        seq![lo]
    } else {
        seq![lo].add(bounded_range((lo + 1) as nat, hi))
    }
}


pub proof fn bounded_range_contains_endpoints(lo: nat, hi: nat)
    requires lo <= hi
    ensures bounded_range(lo, hi).len() > 0,
            bounded_range(lo, hi)[0] == lo
    decreases hi - lo
{
    if lo == hi {
        assert(bounded_range(lo, hi) =~= seq![lo]);
    } else {
        assert(bounded_range(lo, hi) =~= seq![lo].add(bounded_range((lo + 1) as nat, hi)));
        assert(bounded_range(lo, hi)[0] == lo);
    }
}

} // verus!