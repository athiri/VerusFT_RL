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


pub proof fn shrink_nat_until_minimal_result(n: nat, max_iter: nat)
    ensures shrink_nat_until_minimal(n, max_iter) <= n
    decreases max_iter
{
    if max_iter == 0 || n == 0 {
        // Base cases
    } else {
        let shrunk = shrink_nat(n);
        if shrunk.len() == 0 {
            // n doesn't shrink
        } else {
            shrink_nat_smaller(n, 0);
            assert(shrunk[0] < n);
            shrink_nat_until_minimal_result(shrunk[0], (max_iter - 1) as nat);
        }
    }
}

} // verus!