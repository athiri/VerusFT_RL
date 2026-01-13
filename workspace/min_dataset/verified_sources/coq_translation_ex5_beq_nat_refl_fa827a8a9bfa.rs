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


pub proof fn ex5_beq_nat_refl(n: nat)
    ensures beq_nat(n, n)
    decreases n
{
    if n == 0 {
        assert(beq_nat(0, 0));
    } else {
        let n1 = (n - 1) as nat;
        ex5_beq_nat_refl(n1);
        assert(n != 0);
        assert(beq_nat(n, n) == beq_nat(n1, n1));
    }
}

} // verus!