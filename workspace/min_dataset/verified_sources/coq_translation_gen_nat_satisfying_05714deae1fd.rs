use vstd::prelude::*;

verus! {

pub open spec fn gen_nat(seed: nat, size: nat) -> nat {
    if size == 0 {
        0
    } else {
        seed % size
    }
}


pub open spec fn gen_nat_satisfying(
    pred: spec_fn(nat) -> bool,
    seed: nat,
    size: nat,
    attempts: nat
) -> Option<nat>
    decreases attempts
{
    if attempts == 0 {
        Option::None
    } else {
        let candidate = gen_nat(seed, size);
        if pred(candidate) {
            Option::Some(candidate)
        } else {
            gen_nat_satisfying(pred, seed + 1, size, (attempts - 1) as nat)
        }
    }
}

} // verus!