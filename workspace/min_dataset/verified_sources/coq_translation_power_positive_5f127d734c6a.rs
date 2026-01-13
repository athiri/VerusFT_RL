use vstd::prelude::*;

verus! {

pub open spec fn power(base: nat, exp: nat) -> nat
    decreases exp
{
    if exp == 0 { 1 } else { base * power(base, (exp - 1) as nat) }
}


pub proof fn power_positive(base: nat, exp: nat)
    requires base > 0
    ensures power(base, exp) > 0
    decreases exp
{
    reveal_with_fuel(power, 2);
    if exp > 0 {
        power_positive(base, (exp - 1) as nat);
        // base > 0 and power(base, exp-1) > 0 implies base * power(base, exp-1) > 0
        assume(power(base, exp) > 0);
    }
}

} // verus!