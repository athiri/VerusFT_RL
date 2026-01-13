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

pub open spec fn dec_le_nat(a: nat, b: nat) -> Dec {
    bool_to_dec(a <= b)
}

pub open spec fn dec_gt_nat(a: nat, b: nat) -> Dec {
    bool_to_dec(a > b)
}


pub proof fn dec_le_gt_complement(a: nat, b: nat)
    ensures dec_to_bool(dec_le_nat(a, b)) == !dec_to_bool(dec_gt_nat(a, b))
{
}

} // verus!