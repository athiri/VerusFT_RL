use vstd::prelude::*;

verus! {

pub open spec fn mul(n: nat, m: nat) -> nat
    decreases n
{
    if n == 0 {
        0
    } else {
        add(mul((n - 1) as nat, m), m)
    }
}

} // verus!