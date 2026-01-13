use vstd::prelude::*;

verus! {

pub enum Decidable {
    Yes,  // Proposition holds
    No,   // Proposition does not hold
}

pub open spec fn dec_to_bool(d: Decidable) -> bool {
    match d {
        Decidable::Yes => true,
        Decidable::No => false,
    }
}

pub open spec fn dec_or(d1: Decidable, d2: Decidable) -> Decidable {
    match (d1, d2) {
        (Decidable::No, Decidable::No) => Decidable::No,
        _ => Decidable::Yes,
    }
}


pub proof fn dec_or_sound(d1: Decidable, d2: Decidable)
    ensures dec_to_bool(dec_or(d1, d2)) == (dec_to_bool(d1) || dec_to_bool(d2))
{
}

} // verus!