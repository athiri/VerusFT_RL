use vstd::prelude::*;

verus! {

pub open spec fn exists_range_helper(lo: nat, hi: nat, p: spec_fn(nat) -> bool, i: nat) -> bool
    decreases hi - i when i <= hi
{
    if i >= hi {
        false
    } else if p(i) {
        true
    } else {
        exists_range_helper(lo, hi, p, i + 1)
    }
}

} // verus!