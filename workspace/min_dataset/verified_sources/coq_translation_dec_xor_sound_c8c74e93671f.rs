use vstd::prelude::*;

verus! {

pub open spec fn dec_xor(d1: Dec, d2: Dec) -> Dec {
    match (d1, d2) {
        (Dec::Yes, Dec::No) => Dec::Yes,
        (Dec::No, Dec::Yes) => Dec::Yes,
        _ => Dec::No,
    }
}


pub enum Dec {
    Yes,
    No,
}

pub open spec fn dec_to_bool(d: Dec) -> bool {
    match d {
        Dec::Yes => true,
        Dec::No => false,
    }
}


pub proof fn dec_xor_sound(d1: Dec, d2: Dec)
    ensures dec_to_bool(dec_xor(d1, d2)) == (dec_to_bool(d1) != dec_to_bool(d2))
{
}

} // verus!