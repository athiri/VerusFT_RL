use vstd::prelude::*;

verus! {

pub open spec fn shrink_nat_candidates(n: nat) -> Seq<nat> {
    if n == 0 {
        Seq::empty()
    } else if n == 1 {
        seq![0]
    } else {
        seq![0 as nat, n / 2, (n - 1) as nat]
    }
}


pub proof fn shrink_candidates_are_smaller(n: nat)
    ensures forall|i: int| 0 <= i < shrink_nat_candidates(n).len() ==>
        shrink_nat_candidates(n)[i] < n
{
    if n == 0 {
    } else if n == 1 {
        assert(shrink_nat_candidates(n).len() == 1);
        assert(shrink_nat_candidates(n)[0] == 0);
    } else {
        assert(shrink_nat_candidates(n).len() == 3);
        assert(shrink_nat_candidates(n)[0] == 0);
        assert(shrink_nat_candidates(n)[1] == n / 2);
        assert(shrink_nat_candidates(n)[2] == n - 1);
    }
}

} // verus!