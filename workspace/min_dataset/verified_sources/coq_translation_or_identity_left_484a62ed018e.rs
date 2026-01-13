use vstd::prelude::*;

verus! {

pub open spec fn default_bool() -> bool {
    false
}


pub proof fn or_identity_left(x: bool)
    ensures (default_bool() || x) == x
{
    assert(false || x == x);
}

} // verus!