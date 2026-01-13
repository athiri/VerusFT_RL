use vstd::prelude::*;

verus! {

pub open spec fn add(a: nat, b: nat) -> nat
    decreases a
{
    if a == 0 { b } else { add((a - 1) as nat, b) + 1 }
}

} // verus!