use vstd::prelude::*;

verus! {

pub open spec fn dec_and(d1: Dec, d2: Dec) -> Dec {
    match (d1, d2) {
        (Dec::Yes, Dec::Yes) => Dec::Yes,
        _ => Dec::No,
    }
}

pub open spec fn dec_or(d1: Dec, d2: Dec) -> Dec {
    match (d1, d2) {
        (Dec::No, Dec::No) => Dec::No,
        _ => Dec::Yes,
    }
}


pub enum Dec {
    Yes,
    No,
}


pub proof fn dec_or_and_distrib_left(d1: Dec, d2: Dec, d3: Dec)
    ensures dec_or(d1, dec_and(d2, d3)) == dec_and(dec_or(d1, d2), dec_or(d1, d3))
{
}

} // verus!