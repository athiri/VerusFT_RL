use vstd::prelude::*;

verus! {

pub open spec fn arbitrary_nat(seed: nat, size: nat) -> nat {
    if size == 0 {
        0
    } else {
        seed % size
    }
}


pub proof fn arbitrary_nat_in_range(seed: nat, size: nat)
    requires size > 0
    ensures arbitrary_nat(seed, size) < size
{
    assert(seed % size < size);
}

} // verus!