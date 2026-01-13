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

pub open spec fn nat_measure(n: nat) -> nat {
    n
}

pub proof fn shrink_nat_smaller(n: nat, i: int)
    requires 0 <= i < shrink_nat(n).len() as int
    ensures shrink_nat(n)[i] < n
{
    if n == 0 {
        // Vacuously true - empty sequence
    } else {
        let half = (n / 2) as nat;
        if half == 0 {
            assert(shrink_nat(n) =~= seq![0nat]);
            assert(shrink_nat(n)[0] == 0);
            assert(0 < n);
        } else {
            assert(shrink_nat(n) =~= seq![0nat, half]);
            if i == 0 {
                assert(shrink_nat(n)[0] == 0);
                assert(0 < n);
            } else {
                assert(shrink_nat(n)[1] == half);
                assert(half < n);
            }
        }
    }
}


pub proof fn shrink_nat_decreases_measure(n: nat, i: int)
    requires 0 <= i < shrink_nat(n).len() as int
    ensures nat_measure(shrink_nat(n)[i]) < nat_measure(n)
{
    shrink_nat_smaller(n, i);
}

} // verus!