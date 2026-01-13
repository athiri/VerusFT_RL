use vstd::prelude::*;

verus! {

pub open spec fn greedy_shrink_nat(n: nat, prop: spec_fn(nat) -> bool) -> nat
    decreases n
{
    if n == 0 || prop(n) {
        n
    } else {
        let half = n / 2;
        if !prop(half) {
            greedy_shrink_nat(half, prop)
        } else {
            greedy_shrink_nat((n - 1) as nat, prop)
        }
    }
}

} // verus!