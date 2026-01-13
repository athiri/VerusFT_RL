use vstd::prelude::*;

verus! {

pub open spec fn gen_nat(seed: nat, size: nat) -> nat {
    if size == 0 {
        0
    } else {
        seed % size
    }
}


pub open spec fn gen_sample(size: nat, count: nat) -> Seq<nat>
    decreases count
{
    if count == 0 {
        Seq::empty()
    } else {
        let seed = count;  // Use count as seed for variety
        seq![gen_nat(seed, size)].add(gen_sample(size, (count - 1) as nat))
    }
}


pub proof fn gen_sample_length(size: nat, count: nat)
    ensures gen_sample(size, count).len() == count
    decreases count
{
    if count == 0 {
        assert(gen_sample(size, 0) =~= Seq::empty());
    } else {
        gen_sample_length(size, (count - 1) as nat);
    }
}

} // verus!