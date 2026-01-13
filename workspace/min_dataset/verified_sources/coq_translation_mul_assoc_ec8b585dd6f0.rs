use vstd::prelude::*;

verus! {

pub proof fn mul_assoc(a: nat, b: nat, c: nat)
    ensures (a * b) * c == a * (b * c)
{
    // Associativity of multiplication - axiom in Verus
    assert((a * b) * c == a * (b * c)) by (nonlinear_arith);
}

} // verus!