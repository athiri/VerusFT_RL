use vstd::prelude::*;

verus! {

pub open spec fn eq_nat(x: nat, y: nat) -> bool { x == y }


pub proof fn lemma_exists_witness(n: nat)
    ensures exists|m: nat| #[trigger] eq_nat(m, n)
{
    assert(exists|m: nat| #[trigger] eq_nat(m, n)) by {
        let m = n;
        assert(eq_nat(m, n));
    }
}

} // verus!