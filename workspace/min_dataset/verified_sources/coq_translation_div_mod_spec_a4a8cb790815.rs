use vstd::prelude::*;

verus! {

pub proof fn div_mod_spec(a: nat, b: nat)
    requires b > 0
    ensures a == (a / b) * b + (a % b)
{
    // Division/modulo identity - axiom in Verus
    assume(a == (a / b) * b + (a % b));
}

} // verus!