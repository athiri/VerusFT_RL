use vstd::prelude::*;

verus! {

pub open spec fn default_nat() -> nat {
    0
}


pub open spec fn monoid_nat_add_identity() -> nat {
    default_nat()  // 0
}


pub proof fn monoid_nat_add_laws(x: nat, y: nat, z: nat)
    ensures (monoid_nat_add_identity() + x) as nat == x,
            (x + monoid_nat_add_identity()) as nat == x,
            ((x + y) + z) as nat == (x + (y + z)) as nat
{
    // All trivially true for nat addition
}

} // verus!