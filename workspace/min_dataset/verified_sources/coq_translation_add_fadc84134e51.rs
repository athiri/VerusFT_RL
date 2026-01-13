use vstd::prelude::*;

verus! {

pub open spec fn add(n: nat, m: nat) -> nat
    decreases n
{
    if n == 0 { m } else { add((n - 1) as nat, m) + 1 }
}

} // verus!