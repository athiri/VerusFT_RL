use vstd::prelude::*;

verus! {

pub open spec fn fib(n: nat) -> nat
    decreases n
{
    if n == 0 { 0 }
    else if n == 1 { 1 }
    else { fib((n - 1) as nat) + fib((n - 2) as nat) }
}


pub proof fn fib_monotonic(n: nat)
    requires n >= 1
    ensures fib(n) >= fib((n - 1) as nat)
    decreases n
{
    reveal_with_fuel(fib, 3);
    if n > 1 {
        // fib(n) = fib(n-1) + fib(n-2) >= fib(n-1)
    }
}

} // verus!