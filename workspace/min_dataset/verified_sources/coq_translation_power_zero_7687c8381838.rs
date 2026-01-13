use vstd::prelude::*;

verus! {

pub open spec fn power(base: nat, exp: nat) -> nat
    decreases exp
{
    if exp == 0 { 1 } else { base * power(base, (exp - 1) as nat) }
}


pub proof fn power_zero(base: nat)
    ensures power(base, 0) == 1
{
    reveal_with_fuel(power, 2);
}

} // verus!