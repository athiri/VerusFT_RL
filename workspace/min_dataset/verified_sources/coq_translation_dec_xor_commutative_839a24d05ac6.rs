use vstd::prelude::*;

verus! {

pub enum Dec {
    Yes,
    No,
}

pub open spec fn dec_xor(d1: Dec, d2: Dec) -> Dec {
    match (d1, d2) {
        (Dec::Yes, Dec::No) => Dec::Yes,
        (Dec::No, Dec::Yes) => Dec::Yes,
        _ => Dec::No,
    }
}


pub proof fn dec_xor_commutative(d1: Dec, d2: Dec)
    ensures dec_xor(d1, d2) == dec_xor(d2, d1)
{
}

} // verus!