use vstd::prelude::*;

verus! {

pub open spec fn default_nat() -> nat {
    0
}


pub proof fn default_nat_add_right_identity(x: nat)
    ensures (x + default_nat()) as nat == x
{
    assert(x + 0 == x);
}

} // verus!