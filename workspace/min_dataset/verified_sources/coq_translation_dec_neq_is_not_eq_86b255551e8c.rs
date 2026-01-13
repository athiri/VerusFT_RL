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

pub open spec fn dec_neq_nat(a: nat, b: nat) -> Dec {
    bool_to_dec(a != b)
}


pub proof fn dec_neq_is_not_eq(a: nat, b: nat)
    ensures dec_to_bool(dec_neq_nat(a, b)) == !dec_to_bool(dec_eq_nat(a, b))
{
}

} // verus!