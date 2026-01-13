use vstd::prelude::*;

verus! {

pub open spec fn fib(n: nat) -> nat
    decreases n
{
    if n == 0 { 0 }
    else if n == 1 { 1 }
    else { fib((n - 1) as nat) + fib((n - 2) as nat) }
}


pub proof fn fib_nonneg(n: nat)
    ensures fib(n) >= 0
{
    // Trivially true since fib returns nat
}

} // verus!