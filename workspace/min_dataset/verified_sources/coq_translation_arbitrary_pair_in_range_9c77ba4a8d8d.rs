use vstd::prelude::*;

verus! {

pub open spec fn arbitrary_nat(seed: nat, size: nat) -> nat {
    if size == 0 {
        0
    } else {
        seed % size
    }
}


pub open spec fn arbitrary_pair_nat(seed: nat, size: nat) -> (nat, nat) {
    (arbitrary_nat(seed, size), arbitrary_nat(seed + 1, size))
}

pub proof fn arbitrary_nat_in_range(seed: nat, size: nat)
    requires size > 0
    ensures arbitrary_nat(seed, size) < size
{
    assert(seed % size < size);
}


pub proof fn arbitrary_pair_in_range(seed: nat, size: nat)
    requires size > 0
    ensures arbitrary_pair_nat(seed, size).0 < size,
            arbitrary_pair_nat(seed, size).1 < size
{
    arbitrary_nat_in_range(seed, size);
    arbitrary_nat_in_range(seed + 1, size);
}

} // verus!