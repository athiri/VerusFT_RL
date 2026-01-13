use vstd::prelude::*;

verus! {

pub open spec fn exists_nat_lt_helper(n: nat, p: spec_fn(nat) -> bool, i: nat) -> bool
    decreases n - i when i <= n
{
    if i >= n {
        false
    } else if p(i) {
        true
    } else {
        exists_nat_lt_helper(n, p, i + 1)
    }
}

} // verus!