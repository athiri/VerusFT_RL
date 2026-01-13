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

} // verus!