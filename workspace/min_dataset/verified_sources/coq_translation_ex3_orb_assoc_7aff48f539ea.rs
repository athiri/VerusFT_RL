use vstd::prelude::*;

verus! {

pub open spec fn orb(b1: bool, b2: bool) -> bool {
    b1 || b2
}


pub proof fn ex3_orb_assoc(a: bool, b: bool, c: bool)
    ensures orb(a, orb(b, c)) == orb(orb(a, b), c)
{
    // Pure boolean algebra; case split keeps it solver-friendly.
    match (a, b, c) {
        (false, false, false) => {}
        (false, false, true) => {}
        (false, true, false) => {}
        (false, true, true) => {}
        (true, false, false) => {}
        (true, false, true) => {}
        (true, true, false) => {}
        (true, true, true) => {}
    }
}

} // verus!