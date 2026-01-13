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

pub open spec fn dec_and(d1: Decidable, d2: Decidable) -> Decidable {
    match (d1, d2) {
        (Decidable::Yes, Decidable::Yes) => Decidable::Yes,
        _ => Decidable::No,
    }
}


pub proof fn dec_and_sound(d1: Decidable, d2: Decidable)
    ensures dec_to_bool(dec_and(d1, d2)) == (dec_to_bool(d1) && dec_to_bool(d2))
{
}

} // verus!