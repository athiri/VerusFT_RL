use vstd::prelude::*;

verus! {

pub open spec fn forall_nat_lt_helper(n: nat, p: spec_fn(nat) -> bool, i: nat) -> bool
    decreases n - i when i <= n
{
    if i >= n {
        true
    } else if !p(i) {
        false
    } else {
        forall_nat_lt_helper(n, p, i + 1)
    }
}

} // verus!