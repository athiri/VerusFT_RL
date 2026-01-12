// Source: verification/vstd_extra/src/extra_num.rs
// Concept: Computing GCD using Euclidean algorithm
// This example demonstrates:
// - A spec function defining GCD recursively
// - An executable function computing GCD iteratively
// - Loop invariant maintaining GCD preservation

use vstd::prelude::*;

verus! {

/// Spec function: compute GCD using Euclidean algorithm recursively
/// Base case: gcd(a, 0) = a
/// Recursive case: gcd(a, b) = gcd(b, a % b)
pub open spec fn gcd_spec(a: nat, b: nat) -> nat
    decreases b,
{
    if b == 0 {
        a
    } else {
        gcd_spec(b, (a % b) as nat)
    }
}

/// Executable function: compute GCD using iterative Euclidean algorithm
/// Repeatedly replaces (a, b) with (b, a % b) until b == 0
pub fn gcd(a: u64, b: u64) -> (res: u64)
    requires
        a > 0 || b > 0,
    ensures
        res as nat == gcd_spec(a as nat, b as nat),
        res > 0,
{
    let mut x: u64 = a;
    let mut y: u64 = b;
    
    while y != 0
        invariant
            gcd_spec(x as nat, y as nat) == gcd_spec(a as nat, b as nat),
            x > 0 || y > 0,
        decreases y,
    {
        let temp: u64 = y;
        y = x % y;
        x = temp;
    }
    
    assert(gcd_spec(x as nat, 0) == x as nat);
    x
}

fn main() {
    let g1 = gcd(48, 18);
    assert(g1 == gcd_spec(48, 18));
    
    let g2 = gcd(100, 25);
    assert(g2 == gcd_spec(100, 25));
    
    let g3 = gcd(17, 13);
    assert(g3 == gcd_spec(17, 13));
    
    let g4 = gcd(5, 0);
    assert(g4 == gcd_spec(5, 0));
}

} // verus!
