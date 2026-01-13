use vstd::prelude::*;

verus! {

pub proof fn ex10_nat_nonzero_implies_not_zero(n: nat)
    requires n > 0,
    ensures n != 0
{
    if n == 0 {
        // Impossible branch given precondition.
        assert(false);
    }
}

} // verus!