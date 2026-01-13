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


pub proof fn dec_or_absorbing_yes_right(d: Dec)
    ensures dec_or(d, Dec::Yes) == Dec::Yes
{
}

} // verus!