use vstd::prelude::*;

verus! {

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


pub proof fn dec_or_associative(d1: Dec, d2: Dec, d3: Dec)
    ensures dec_or(dec_or(d1, d2), d3) == dec_or(d1, dec_or(d2, d3))
{
}

} // verus!