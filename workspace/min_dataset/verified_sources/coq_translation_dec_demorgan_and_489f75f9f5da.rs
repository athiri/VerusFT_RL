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

pub open spec fn dec_not(d: Dec) -> Dec {
    match d {
        Dec::Yes => Dec::No,
        Dec::No => Dec::Yes,
    }
}


pub proof fn dec_demorgan_and(d1: Dec, d2: Dec)
    ensures dec_not(dec_and(d1, d2)) == dec_or(dec_not(d1), dec_not(d2))
{
}

} // verus!