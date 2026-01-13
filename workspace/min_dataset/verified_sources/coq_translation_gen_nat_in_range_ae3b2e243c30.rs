use vstd::prelude::*;

verus! {

pub open spec fn gen_nat(seed: nat, size: nat) -> nat {
    if size == 0 {
        0
    } else {
        seed % size
    }
}


pub proof fn gen_nat_in_range(seed: nat, size: nat)
    requires size > 0
    ensures gen_nat(seed, size) < size
{
    assert(seed % size < size);
}

} // verus!