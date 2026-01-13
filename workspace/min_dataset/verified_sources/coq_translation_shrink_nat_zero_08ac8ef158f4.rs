use vstd::prelude::*;

verus! {

pub open spec fn shrink_nat(n: nat) -> Seq<nat>
    decreases n
{
    if n == 0 {
        Seq::empty()
    } else {
        let half = (n / 2) as nat;
        if half == 0 {
            seq![0nat]
        } else {
            seq![0nat, half]
        }
    }
}


pub proof fn shrink_nat_zero()
    ensures shrink_nat(0).len() == 0
{
    assert(shrink_nat(0) =~= Seq::empty());
}

} // verus!