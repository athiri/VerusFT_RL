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


pub proof fn ex7_beq_nat_sym(n: nat, m: nat)
    ensures beq_nat(n, m) == beq_nat(m, n)
    decreases n
{
    if n == 0 {
        if m == 0 {
            assert(beq_nat(0, 0));
        } else {
            assert(beq_nat(0, m) == false);
            assert(beq_nat(m, 0) == false);
        }
    } else {
        if m == 0 {
            assert(beq_nat(n, 0) == false);
            assert(beq_nat(0, n) == false);
        } else {
            let n1 = (n - 1) as nat;
            let m1 = (m - 1) as nat;
            ex7_beq_nat_sym(n1, m1);
            assert(beq_nat(n, m) == beq_nat(n1, m1));
            assert(beq_nat(m, n) == beq_nat(m1, n1));
        }
    }
}

} // verus!