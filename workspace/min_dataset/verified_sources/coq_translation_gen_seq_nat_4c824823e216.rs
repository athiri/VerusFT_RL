use vstd::prelude::*;

verus! {

pub open spec fn gen_nat(seed: nat, size: nat) -> nat {
    if size == 0 {
        0
    } else {
        seed % size
    }
}


pub open spec fn gen_seq_nat(seed: nat, size: nat, len: nat) -> Seq<nat>
    decreases len
{
    if len == 0 {
        Seq::empty()
    } else {
        let elem = gen_nat(seed, size);
        seq![elem].add(gen_seq_nat(seed + 1, size, (len - 1) as nat))
    }
}

} // verus!