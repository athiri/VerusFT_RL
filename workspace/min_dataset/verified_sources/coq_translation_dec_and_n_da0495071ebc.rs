use vstd::prelude::*;

verus! {

pub open spec fn dec_and(d1: Dec, d2: Dec) -> Dec {
    match (d1, d2) {
        (Dec::Yes, Dec::Yes) => Dec::Yes,
        _ => Dec::No,
    }
}


pub enum Dec {
    Yes,
    No,
}


pub open spec fn dec_and_n(d: Dec, n: nat) -> Dec
    decreases n
{
    if n == 0 {
        Dec::Yes
    } else {
        dec_and(d, dec_and_n(d, (n - 1) as nat))
    }
}

} // verus!