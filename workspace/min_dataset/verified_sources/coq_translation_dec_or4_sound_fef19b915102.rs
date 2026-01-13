use vstd::prelude::*;

verus! {

pub open spec fn dec_or(d1: Dec, d2: Dec) -> Dec {
    match (d1, d2) {
        (Dec::No, Dec::No) => Dec::No,
        _ => Dec::Yes,
    }
}

pub open spec fn dec_or3(d1: Dec, d2: Dec, d3: Dec) -> Dec {
    dec_or(dec_or(d1, d2), d3)
}


pub open spec fn dec_to_bool(d: Dec) -> bool {
    match d {
        Dec::Yes => true,
        Dec::No => false,
    }
}

pub open spec fn dec_or4(d1: Dec, d2: Dec, d3: Dec, d4: Dec) -> Dec {
    dec_or(dec_or3(d1, d2, d3), d4)
}


pub enum Dec {
    Yes,
    No,
}


pub proof fn dec_or4_sound(d1: Dec, d2: Dec, d3: Dec, d4: Dec)
    ensures dec_to_bool(dec_or4(d1, d2, d3, d4)) ==
        (dec_to_bool(d1) || dec_to_bool(d2) || dec_to_bool(d3) || dec_to_bool(d4))
{
}

} // verus!