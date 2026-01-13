use vstd::prelude::*;

verus! {

pub proof fn div_self(a: nat)
    requires a > 0
    ensures a / a == 1
{
}

} // verus!