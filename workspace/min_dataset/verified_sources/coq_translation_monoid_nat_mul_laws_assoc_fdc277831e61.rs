use vstd::prelude::*;

verus! {

pub proof fn monoid_nat_mul_laws_assoc(x: nat, y: nat, z: nat)
    ensures (x * y) * z == x * (y * z)
{
    // Associativity of nat multiplication
    assert((x * y) * z == x * (y * z)) by(nonlinear_arith);
}

} // verus!