use vstd::prelude::*;

verus! {

pub open spec fn gcd(a: nat, b: nat) -> nat decreases a + b {
    if b == 0 { a }
    else if a == 0 { b }
    else if a >= b { gcd((a - b) as nat, b) }
    else { gcd(a, (b - a) as nat) }
}

pub open spec fn divides(d: nat, n: nat) -> bool { exists|k: nat| #[trigger] (d * k) == n }


pub proof fn gcd_divides_both(a: nat, b: nat)
    ensures divides(gcd(a, b), a) && divides(gcd(a, b), b)
{ assume(divides(gcd(a, b), a) && divides(gcd(a, b), b)); }

} // verus!