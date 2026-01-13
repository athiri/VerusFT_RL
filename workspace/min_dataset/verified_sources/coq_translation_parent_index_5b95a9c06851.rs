use vstd::prelude::*;

verus! {

pub open spec fn parent_index(i: nat) -> nat
    decreases i
{
    if i <= 1 { 0 } else { (i / 2) as nat }
}

} // verus!