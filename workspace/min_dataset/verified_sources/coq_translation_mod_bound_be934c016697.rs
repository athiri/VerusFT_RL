use vstd::prelude::*;

verus! {

pub proof fn mod_bound(a: nat, m: nat)
    requires m > 0
    ensures a % m < m
{}

} // verus!