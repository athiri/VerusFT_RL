use vstd::prelude::*;

verus! {

pub open spec fn factorial(n: nat) -> nat
    decreases n
{
    if n == 0 { 1 } else { n * factorial((n - 1) as nat) }
}

} // verus!