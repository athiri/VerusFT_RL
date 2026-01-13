use vstd::prelude::*;

verus! {

pub open spec fn dec_or(d1: Dec, d2: Dec) -> Dec {
    match (d1, d2) {
        (Dec::No, Dec::No) => Dec::No,
        _ => Dec::Yes,
    }
}


pub open spec fn bool_to_dec(b: bool) -> Dec {
    if b { Dec::Yes } else { Dec::No }
}

pub open spec fn dec_or3(d1: Dec, d2: Dec, d3: Dec) -> Dec {
    dec_or(dec_or(d1, d2), d3)
}


pub enum Dec {
    Yes,
    No,
}

pub open spec fn lift_or3(p1: bool, p2: bool, p3: bool) -> Dec {
    dec_or3(bool_to_dec(p1), bool_to_dec(p2), bool_to_dec(p3))
}

pub open spec fn dec_to_bool(d: Dec) -> bool {
    match d {
        Dec::Yes => true,
        Dec::No => false,
    }
}


pub proof fn lift_or3_sound(p1: bool, p2: bool, p3: bool)
    ensures dec_to_bool(lift_or3(p1, p2, p3)) == (p1 || p2 || p3)
{
}

} // verus!