use vstd::prelude::*;

verus! {

pub proof fn add_assoc(a: nat, b: nat, c: nat)
    ensures (a + b) + c == a + (b + c)
{
}

} // verus!