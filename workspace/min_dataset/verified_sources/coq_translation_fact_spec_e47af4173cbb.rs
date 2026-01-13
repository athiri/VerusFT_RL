use vstd::prelude::*;

verus! {

pub open spec fn fact_spec(n: nat) -> nat
    decreases n
{
    if n == 0 { 1 }
    else { n * fact_spec((n - 1) as nat) }
}

} // verus!