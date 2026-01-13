use vstd::prelude::*;

verus! {

pub open spec fn mul(a: nat, b: nat) -> nat
    decreases a
{
    if a == 0 { 0 } else { add(b, mul((a - 1) as nat, b)) }
}

} // verus!