use vstd::prelude::*;

verus! {

pub enum Decidable {
    Yes,  // Proposition holds
    No,   // Proposition does not hold
}

pub open spec fn bool_to_dec(b: bool) -> Decidable {
    if b { Decidable::Yes } else { Decidable::No }
}

pub open spec fn dec_to_bool(d: Decidable) -> bool {
    match d {
        Decidable::Yes => true,
        Decidable::No => false,
    }
}


pub proof fn dec_roundtrip(b: bool)
    ensures dec_to_bool(bool_to_dec(b)) == b
{
}

} // verus!