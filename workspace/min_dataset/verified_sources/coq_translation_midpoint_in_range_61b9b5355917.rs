use vstd::prelude::*;

verus! {

pub open spec fn midpoint(lo: nat, hi: nat) -> nat
    recommends lo <= hi
{
    (lo + (hi - lo) / 2) as nat
}


pub proof fn midpoint_in_range(lo: nat, hi: nat)
    requires lo <= hi
    ensures lo <= midpoint(lo, hi) && midpoint(lo, hi) <= hi
{
    let mid = lo + (hi - lo) / 2;
    assert((hi - lo) / 2 <= hi - lo);
    assert(mid <= hi);
    assert(mid >= lo);
}

} // verus!