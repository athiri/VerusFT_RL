use vstd::prelude::*;

verus! {

pub open spec fn gcd(a: nat, b: nat) -> nat decreases a + b {
    if b == 0 { a }
    else if a == 0 { b }
    else if a >= b { gcd((a - b) as nat, b) }
    else { gcd(a, (b - a) as nat) }
}


pub proof fn gcd_self(a: nat) ensures gcd(a, a) == a { reveal_with_fuel(gcd, 3); }

} // verus!