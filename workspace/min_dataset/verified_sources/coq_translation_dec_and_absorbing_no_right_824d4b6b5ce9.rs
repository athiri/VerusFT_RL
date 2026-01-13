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


pub proof fn dec_and_absorbing_no_right(d: Dec)
    ensures dec_and(d, Dec::No) == Dec::No
{
}

} // verus!