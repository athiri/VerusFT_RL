use vstd::prelude::*;

verus! {

pub open spec fn dec_or(d1: Dec, d2: Dec) -> Dec {
    match (d1, d2) {
        (Dec::No, Dec::No) => Dec::No,
        _ => Dec::Yes,
    }
}


pub enum Dec {
    Yes,
    No,
}


pub open spec fn dec_or_n(d: Dec, n: nat) -> Dec
    decreases n
{
    if n == 0 {
        Dec::No
    } else {
        dec_or(d, dec_or_n(d, (n - 1) as nat))
    }
}

} // verus!