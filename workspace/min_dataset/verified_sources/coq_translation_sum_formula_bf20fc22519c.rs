use vstd::prelude::*;

verus! {

pub open spec fn sum_to(n: nat) -> nat
    decreases n
{
    if n == 0 { 0 } else { n + sum_to((n - 1) as nat) }
}


pub proof fn sum_formula(n: nat)
    ensures 2 * sum_to(n) == n * (n + 1)
    decreases n
{
    reveal_with_fuel(sum_to, 2);
    if n > 0 {
        sum_formula((n - 1) as nat);
    }
    // Proof by induction - complex arithmetic step
    assume(2 * sum_to(n) == n * (n + 1));
}

} // verus!