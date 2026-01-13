use vstd::prelude::*;

verus! {

pub open spec fn gcd(a: nat, b: nat) -> nat decreases a + b {
    if b == 0 { a }
    else if a == 0 { b }
    else if a >= b { gcd((a - b) as nat, b) }
    else { gcd(a, (b - a) as nat) }
}


pub proof fn gcd_comm(a: nat, b: nat) ensures gcd(a, b) == gcd(b, a) decreases a + b {
    reveal_with_fuel(gcd, 3);
    if b == 0 || a == 0 {} else if a >= b { gcd_comm((a - b) as nat, b); } else { gcd_comm(a, (b - a) as nat); }
}

} // verus!