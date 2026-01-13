use vstd::prelude::*;

verus! {

pub open spec fn bool_to_dec(b: bool) -> Dec {
    if b { Dec::Yes } else { Dec::No }
}


pub enum Dec {
    Yes,
    No,
}

pub open spec fn dec_eq_nat(a: nat, b: nat) -> Dec {
    bool_to_dec(a == b)
}

pub open spec fn dec_to_bool(d: Dec) -> bool {
    match d {
        Dec::Yes => true,
        Dec::No => false,
    }
}


pub proof fn dec_eq_nat_symmetric(x: nat, y: nat)
    ensures dec_to_bool(dec_eq_nat(x, y)) == dec_to_bool(dec_eq_nat(y, x))
{
}

} // verus!