use vstd::prelude::*;

verus! {

pub enum Dec {
    Yes,
    No,
}

pub open spec fn bool_to_dec(b: bool) -> Dec {
    if b { Dec::Yes } else { Dec::No }
}

pub open spec fn dec_to_bool(d: Dec) -> bool {
    match d {
        Dec::Yes => true,
        Dec::No => false,
    }
}


pub proof fn bool_is_decidable(b: bool)
    ensures exists|d: Dec| dec_to_bool(d) == b
{
    let d = bool_to_dec(b);
    assert(dec_to_bool(d) == b);
}

} // verus!