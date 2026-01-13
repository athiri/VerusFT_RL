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


pub proof fn bounded_range_length(lo: nat, hi: nat)
    requires lo <= hi
    ensures bounded_range(lo, hi).len() == (hi - lo + 1) as nat
    decreases hi - lo
{
    if lo == hi {
        assert(bounded_range(lo, hi) =~= seq![lo]);
        assert(bounded_range(lo, hi).len() == 1);
    } else {
        bounded_range_length((lo + 1) as nat, hi);
        assert(bounded_range(lo, hi) =~= seq![lo].add(bounded_range((lo + 1) as nat, hi)));
        assert(bounded_range(lo, hi).len() == 1 + bounded_range((lo + 1) as nat, hi).len());
    }
}

} // verus!