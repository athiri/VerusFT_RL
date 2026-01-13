use vstd::prelude::*;

verus! {

pub open spec fn pow2(n: nat) -> nat
    decreases n
{
    if n == 0 { 1 } else { 2 * pow2((n - 1) as nat) }
}


pub proof fn pow2_monotonic(a: nat, b: nat)
    requires a <= b
    ensures pow2(a) <= pow2(b)
    decreases b
{
    reveal_with_fuel(pow2, 2);
    if a < b {
        pow2_monotonic(a, (b - 1) as nat);
    }
}

} // verus!