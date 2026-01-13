use vstd::prelude::*;

verus! {

pub open spec fn default_pair<A, B>(da: A, db: B) -> (A, B) {
    (da, db)
}


pub open spec fn default_pair_nat() -> (nat, nat) {
    default_pair(default_nat(), default_nat())
}

pub open spec fn default_nat() -> nat {
    0
}


pub proof fn default_pair_nat_is_zeros()
    ensures default_pair_nat() == (0nat, 0nat)
{
    assert(default_nat() == 0);
}

} // verus!