use vstd::prelude::*;

verus! {

pub enum Dec {
    Yes,
    No,
}

pub open spec fn dec_and(d1: Dec, d2: Dec) -> Dec {
    match (d1, d2) {
        (Dec::Yes, Dec::Yes) => Dec::Yes,
        _ => Dec::No,
    }
}


pub proof fn dec_and_commutative(d1: Dec, d2: Dec)
    ensures dec_and(d1, d2) == dec_and(d2, d1)
{
}

} // verus!