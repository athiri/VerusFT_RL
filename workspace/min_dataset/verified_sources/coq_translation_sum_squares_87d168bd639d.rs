use vstd::prelude::*;

verus! {

pub open spec fn sum_squares(n: nat) -> nat
    decreases n
{
    if n == 0 { 0 } else { n * n + sum_squares((n - 1) as nat) }
}

} // verus!