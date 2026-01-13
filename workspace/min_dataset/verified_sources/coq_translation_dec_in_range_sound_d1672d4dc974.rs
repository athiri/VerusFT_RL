use vstd::prelude::*;

verus! {

pub open spec fn bool_to_dec(b: bool) -> Dec {
    if b { Dec::Yes } else { Dec::No }
}


pub enum Dec {
    Yes,
    No,
}

pub open spec fn dec_to_bool(d: Dec) -> bool {
    match d {
        Dec::Yes => true,
        Dec::No => false,
    }
}

pub open spec fn dec_in_range(x: nat, lo: nat, hi: nat) -> Dec {
    bool_to_dec(lo <= x && x < hi)
}


pub proof fn dec_in_range_sound(x: nat, lo: nat, hi: nat)
    ensures dec_to_bool(dec_in_range(x, lo, hi)) <==> (lo <= x && x < hi)
{
}

} // verus!