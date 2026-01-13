use vstd::prelude::*;

verus! {

pub open spec fn default_bool_and_identity() -> bool {
    true
}


pub proof fn and_identity_left(x: bool)
    ensures (default_bool_and_identity() && x) == x
{
    assert(true && x == x);
}

} // verus!