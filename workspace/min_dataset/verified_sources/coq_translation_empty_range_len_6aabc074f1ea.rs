use vstd::prelude::*;

verus! {

pub struct Range { pub lo: nat, pub hi: nat }

pub open spec fn range_len(r: Range) -> nat { if r.lo <= r.hi { (r.hi - r.lo) as nat } else { 0 } }

pub open spec fn range_empty(r: Range) -> bool { r.lo >= r.hi }


pub proof fn empty_range_len(r: Range) requires range_empty(r) ensures range_len(r) == 0 {}

} // verus!