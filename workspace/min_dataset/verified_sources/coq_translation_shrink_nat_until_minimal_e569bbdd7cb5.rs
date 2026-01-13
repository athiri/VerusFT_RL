use vstd::prelude::*;

verus! {

pub open spec fn shrink_nat(n: nat) -> Seq<nat> {
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


pub open spec fn shrink_nat_until_minimal(n: nat, max_iter: nat) -> nat
    decreases max_iter
{
    if max_iter == 0 {
        n
    } else if n == 0 {
        0
    } else {
        let shrunk = shrink_nat(n);
        if shrunk.len() == 0 {
            n
        } else {
            shrink_nat_until_minimal(shrunk[0], (max_iter - 1) as nat)
        }
    }
}

} // verus!