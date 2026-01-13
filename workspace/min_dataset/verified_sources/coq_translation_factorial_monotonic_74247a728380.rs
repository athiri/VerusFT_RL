use vstd::prelude::*;

verus! {

pub open spec fn factorial(n: nat) -> nat
    decreases n
{
    if n == 0 { 1 } else { n * factorial((n - 1) as nat) }
}


pub proof fn factorial_monotonic(n: nat)
    requires n > 0
    ensures factorial(n) >= factorial((n - 1) as nat)
    decreases n
{
    reveal_with_fuel(factorial, 2);
    // n * (n-1)! >= (n-1)! when n > 0
    assume(factorial(n) >= factorial((n - 1) as nat));
}

} // verus!