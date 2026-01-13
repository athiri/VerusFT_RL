use vstd::prelude::*;

verus! {

pub open spec fn shrink_size(size: nat, factor: nat) -> nat
    recommends factor > 0
{
    if factor == 0 || size == 0 {
        0
    } else {
        ((size - 1) as int / factor as int) as nat
    }
}


pub proof fn shrink_decreases(size: nat, factor: nat)
    requires size > 0, factor > 0
    ensures shrink_size(size, factor) < size
{
}

} // verus!