use vstd::prelude::*;

verus! {

pub open spec fn is_even(n: nat) -> bool
    decreases n
{
    if n == 0 {
        true
    } else if n == 1 {
        false
    } else {
        is_even((n - 2) as nat)
    }
}

} // verus!