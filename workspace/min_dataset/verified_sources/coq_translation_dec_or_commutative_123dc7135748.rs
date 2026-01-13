use vstd::prelude::*;

verus! {

pub enum Dec {
    Yes,
    No,
}

pub open spec fn dec_or(d1: Dec, d2: Dec) -> Dec {
    match (d1, d2) {
        (Dec::No, Dec::No) => Dec::No,
        _ => Dec::Yes,
    }
}


pub proof fn dec_or_commutative(d1: Dec, d2: Dec)
    ensures dec_or(d1, d2) == dec_or(d2, d1)
{
}

} // verus!