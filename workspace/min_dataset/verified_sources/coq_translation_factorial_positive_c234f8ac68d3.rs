use vstd::prelude::*;

verus! {

pub open spec fn factorial(n: nat) -> nat
    decreases n
{
    if n == 0 { 1 } else { n * factorial((n - 1) as nat) }
}


pub proof fn factorial_positive(n: nat)
    ensures factorial(n) > 0
    decreases n
{
    reveal_with_fuel(factorial, 2);
    if n > 0 {
        factorial_positive((n - 1) as nat);
        assert(factorial(n) > 0) by (nonlinear_arith)
            requires factorial((n - 1) as nat) > 0,
                     factorial(n) == n * factorial((n - 1) as nat),
                     n > 0;
    }
}

} // verus!