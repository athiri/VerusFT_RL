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


pub proof fn shrink_nat_smaller(n: nat, i: int)
    requires n > 0,
             0 <= i < shrink_nat(n).len() as int
    ensures shrink_nat(n)[i] < n
{
    let half = (n / 2) as nat;
    if half == 0 {
        assert(shrink_nat(n) =~= seq![0nat]);
        assert(shrink_nat(n)[0] == 0 < n);
    } else {
        if i == 0 {
            assert(shrink_nat(n)[0] == 0 < n);
        } else {
            assert(shrink_nat(n)[1] == half < n);
        }
    }
}

} // verus!