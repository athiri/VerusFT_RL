use vstd::prelude::*;

verus! {

pub open spec fn dec_and(d1: Dec, d2: Dec) -> Dec {
    match (d1, d2) {
        (Dec::Yes, Dec::Yes) => Dec::Yes,
        _ => Dec::No,
    }
}


pub open spec fn bool_to_dec(b: bool) -> Dec {
    if b { Dec::Yes } else { Dec::No }
}

pub open spec fn dec_and3(d1: Dec, d2: Dec, d3: Dec) -> Dec {
    dec_and(dec_and(d1, d2), d3)
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

pub open spec fn lift_and3(p1: bool, p2: bool, p3: bool) -> Dec {
    dec_and3(bool_to_dec(p1), bool_to_dec(p2), bool_to_dec(p3))
}


pub proof fn lift_and3_sound(p1: bool, p2: bool, p3: bool)
    ensures dec_to_bool(lift_and3(p1, p2, p3)) == (p1 && p2 && p3)
{
}

} // verus!