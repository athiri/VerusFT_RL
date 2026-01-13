use vstd::prelude::*;

verus! {

pub open spec fn bool_to_dec(b: bool) -> Dec {
    if b { Dec::Yes } else { Dec::No }
}


pub enum Ordering {
    Lt,
    Eq,
    Gt,
}

pub open spec fn dec_cmp_nat(a: nat, b: nat) -> Ordering {
    if a < b { Ordering::Lt }
    else if a == b { Ordering::Eq }
    else { Ordering::Gt }
}

pub enum Dec {
    Yes,
    No,
}

pub open spec fn dec_lt_nat(a: nat, b: nat) -> Dec {
    bool_to_dec(a < b)
}

pub open spec fn dec_to_bool(d: Dec) -> bool {
    match d {
        Dec::Yes => true,
        Dec::No => false,
    }
}


pub proof fn dec_cmp_consistent_lt(a: nat, b: nat)
    ensures (dec_cmp_nat(a, b) == Ordering::Lt) <==> dec_to_bool(dec_lt_nat(a, b))
{
}

} // verus!