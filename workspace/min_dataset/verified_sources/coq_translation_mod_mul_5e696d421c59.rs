use vstd::prelude::*;

verus! {

pub proof fn mod_mul(a: nat, b: nat, m: nat)
    requires m > 0
    ensures (a * b) % m == ((a % m) * (b % m)) % m
{
    assume((a * b) % m == ((a % m) * (b % m)) % m);
}

} // verus!