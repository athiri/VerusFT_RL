use vstd::prelude::*;

verus! {

pub open spec fn dec_and(d1: Dec, d2: Dec) -> Dec {
    match (d1, d2) {
        (Dec::Yes, Dec::Yes) => Dec::Yes,
        _ => Dec::No,
    }
}


pub enum Dec {
    Yes,
    No,
}


pub proof fn dec_and_associative(d1: Dec, d2: Dec, d3: Dec)
    ensures dec_and(dec_and(d1, d2), d3) == dec_and(d1, dec_and(d2, d3))
{
}

} // verus!