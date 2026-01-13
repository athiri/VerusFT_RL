use vstd::prelude::*;

verus! {

pub open spec fn gen_even_nat(seed: nat, size: nat) -> nat {
    let n = gen_nat(seed, size);
    (n - n % 2) as nat  // Round down to even
}

pub open spec fn gen_nat(seed: nat, size: nat) -> nat {
    if size == 0 {
        0
    } else {
        seed % size
    }
}


pub proof fn gen_even_nat_is_even(seed: nat, size: nat)
    ensures gen_even_nat(seed, size) % 2 == 0
{
    let n = gen_nat(seed, size);
    let result = n - n % 2;
    assert(result % 2 == 0);
}

} // verus!