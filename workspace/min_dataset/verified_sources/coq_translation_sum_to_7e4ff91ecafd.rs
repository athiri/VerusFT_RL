use vstd::prelude::*;

verus! {

pub open spec fn sum_to(n: nat) -> nat
    decreases n
{
    if n == 0 { 0 } else { n + sum_to((n - 1) as nat) }
}

} // verus!