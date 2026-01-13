use vstd::prelude::*;

verus! {

pub open spec fn bool_to_dec(b: bool) -> Dec {
    if b { Dec::Yes } else { Dec::No }
}


pub enum Dec {
    Yes,
    No,
}

pub open spec fn dec_implies_bool(a: bool, b: bool) -> Dec {
    bool_to_dec(!a || b)
}

pub open spec fn dec_to_bool(d: Dec) -> bool {
    match d {
        Dec::Yes => true,
        Dec::No => false,
    }
}


pub proof fn dec_implies_bool_sound(a: bool, b: bool)
    ensures dec_to_bool(dec_implies_bool(a, b)) == (!a || b)
{
}

} // verus!