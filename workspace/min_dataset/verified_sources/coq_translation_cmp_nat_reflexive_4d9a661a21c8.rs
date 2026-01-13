use vstd::prelude::*;

verus! {

pub enum Ordering {
    Lt,
    Eq,
    Gt,
}

pub open spec fn cmp_nat(a: nat, b: nat) -> Ordering {
    if a < b { Ordering::Lt }
    else if a == b { Ordering::Eq }
    else { Ordering::Gt }
}


pub proof fn cmp_nat_reflexive(x: nat)
    ensures cmp_nat(x, x) == Ordering::Eq
{
}

} // verus!