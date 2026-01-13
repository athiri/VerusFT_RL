use vstd::prelude::*;

verus! {

pub open spec fn power(base: nat, exp: nat) -> nat
    decreases exp
{
    if exp == 0 { 1 } else { base * power(base, (exp - 1) as nat) }
}


pub proof fn power_add(base: nat, m: nat, n: nat)
    ensures power(base, m + n) == power(base, m) * power(base, n)
    decreases m
{
    reveal_with_fuel(power, 2);
    if m > 0 {
        power_add(base, (m - 1) as nat, n);
    }
    // By IH and definition of power
    assume(power(base, m + n) == power(base, m) * power(base, n));
}

} // verus!