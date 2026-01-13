use vstd::prelude::*;

verus! {

pub open spec fn arbitrary_nat(seed: nat, size: nat) -> nat {
    if size == 0 {
        0
    } else {
        seed % size
    }
}


pub open spec fn arbitrary_seq_nat(seed: nat, size: nat) -> Seq<nat>
    decreases size
{
    if size == 0 {
        Seq::empty()
    } else {
        let len = seed % (size + 1);
        Seq::new(len, |i: int| arbitrary_nat(seed + i as nat + 1, size))
    }
}


pub proof fn arbitrary_seq_bounded_len(seed: nat, size: nat)
    ensures arbitrary_seq_nat(seed, size).len() <= size
{
    if size == 0 {
        assert(arbitrary_seq_nat(seed, 0) =~= Seq::empty());
    } else {
        let len = seed % (size + 1);
        assert(len <= size);
    }
}

} // verus!