use vstd::prelude::*;

verus! {

pub open spec fn default_nat() -> nat {
    0
}


pub proof fn default_nat_add_left_identity(x: nat)
    ensures (default_nat() + x) as nat == x
{
    assert(0 + x == x);
}

} // verus!