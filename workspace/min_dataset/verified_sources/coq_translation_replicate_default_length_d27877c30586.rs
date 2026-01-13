use vstd::prelude::*;

verus! {

pub open spec fn default_nat() -> nat {
    0
}


pub open spec fn replicate_default_nat(n: nat) -> Seq<nat>
    decreases n
{
    if n == 0 {
        Seq::empty()
    } else {
        seq![default_nat()].add(replicate_default_nat((n - 1) as nat))
    }
}


pub proof fn replicate_default_length(n: nat)
    ensures replicate_default_nat(n).len() == n
    decreases n
{
    if n == 0 {
        assert(replicate_default_nat(0) =~= Seq::empty());
    } else {
        replicate_default_length((n - 1) as nat);
        assert(replicate_default_nat(n).len() == 1 + replicate_default_nat((n - 1) as nat).len());
    }
}

} // verus!