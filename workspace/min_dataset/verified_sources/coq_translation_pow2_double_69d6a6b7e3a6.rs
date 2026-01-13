use vstd::prelude::*;

verus! {

pub open spec fn pow2(n: nat) -> nat
    decreases n
{
    if n == 0 { 1 } else { 2 * pow2((n - 1) as nat) }
}


pub proof fn pow2_double(n: nat)
    ensures pow2(n) + pow2(n) == pow2(n + 1)
{
    reveal_with_fuel(pow2, 2);
}

} // verus!