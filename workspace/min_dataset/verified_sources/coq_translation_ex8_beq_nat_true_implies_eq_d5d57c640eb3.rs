use vstd::prelude::*;

verus! {

pub open spec fn beq_nat(n: nat, m: nat) -> bool
    decreases n
{
    if n == 0 {
        m == 0
    } else if m == 0 {
        false
    } else {
        beq_nat((n - 1) as nat, (m - 1) as nat)
    }
}


pub proof fn ex8_beq_nat_true_implies_eq(n: nat, m: nat)
    requires beq_nat(n, m)
    ensures n == m
    decreases n
{
    if n == 0 {
        assert(m == 0);
    } else {
        if m == 0 {
            // beq_nat(n,0) is false when n>0, contradicting the precondition.
            assert(beq_nat(n, 0) == false);
            assert(false);
        } else {
            let n1 = (n - 1) as nat;
            let m1 = (m - 1) as nat;
            assert(beq_nat(n, m) == beq_nat(n1, m1));
            ex8_beq_nat_true_implies_eq(n1, m1);
            assert(n == n1 + 1);
            assert(m == m1 + 1);
        }
    }
}

} // verus!