use vstd::prelude::*;

verus! {

pub open spec fn fib(n: nat) -> nat
    decreases n
{
    if n == 0 { 0 }
    else if n == 1 { 1 }
    else { fib((n - 1) as nat) + fib((n - 2) as nat) }
}


pub proof fn fib_lower_bound(n: nat)
    requires n >= 1
    ensures fib(n) >= (n - 1) as nat
    decreases n
{
    reveal_with_fuel(fib, 3);
    if n > 2 {
        fib_lower_bound((n - 1) as nat);
        fib_lower_bound((n - 2) as nat);
    }
}

} // verus!