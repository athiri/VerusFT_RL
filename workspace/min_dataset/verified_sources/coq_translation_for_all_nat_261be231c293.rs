use vstd::prelude::*;

verus! {

pub open spec fn arbitrary_nat(seed: nat, size: nat) -> nat {
    if size == 0 {
        0
    } else {
        seed % size
    }
}


pub open spec fn for_all_nat(
    prop: spec_fn(nat) -> bool,
    seed: nat,
    size: nat,
    num_tests: nat
) -> bool
    decreases num_tests
{
    if num_tests == 0 {
        true
    } else {
        let val = arbitrary_nat(seed, size);
        if !prop(val) {
            false
        } else {
            for_all_nat(prop, seed + 1, size, (num_tests - 1) as nat)
        }
    }
}

} // verus!