use vstd::prelude::*;

verus! {

pub open spec fn bool_to_dec(b: bool) -> Dec {
    if b { Dec::Yes } else { Dec::No }
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

pub open spec fn lift_or(p1: bool, p2: bool) -> Dec {
    dec_or(bool_to_dec(p1), bool_to_dec(p2))
}

pub open spec fn dec_to_bool(d: Dec) -> bool {
    match d {
        Dec::Yes => true,
        Dec::No => false,
    }
}


pub proof fn lift_or_sound(p1: bool, p2: bool)
    ensures dec_to_bool(lift_or(p1, p2)) == (p1 || p2)
{
}

} // verus!