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

pub open spec fn dec_not(d: Decidable) -> Decidable {
    match d {
        Decidable::Yes => Decidable::No,
        Decidable::No => Decidable::Yes,
    }
}


pub proof fn dec_not_sound(d: Decidable)
    ensures dec_to_bool(dec_not(d)) == !dec_to_bool(d)
{
}

} // verus!