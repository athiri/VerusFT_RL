use vstd::prelude::*;

verus! {

pub open spec fn find_witness_helper(n: nat, p: spec_fn(nat) -> bool, i: nat) -> Option<nat>
    decreases n - i when i <= n
{
    if i >= n {
        Option::None
    } else if p(i) {
        Option::Some(i)
    } else {
        find_witness_helper(n, p, i + 1)
    }
}

} // verus!