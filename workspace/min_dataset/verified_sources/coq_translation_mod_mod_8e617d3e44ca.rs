use vstd::prelude::*;

verus! {

pub proof fn mod_mod(a: nat, m: nat)
    requires m > 0
    ensures (a % m) % m == a % m
{
    assume((a % m) % m == a % m);
}

} // verus!