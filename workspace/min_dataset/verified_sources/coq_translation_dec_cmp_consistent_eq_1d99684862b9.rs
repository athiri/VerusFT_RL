use vstd::prelude::*;

verus! {

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


pub proof fn dec_cmp_consistent_eq(a: nat, b: nat)
    ensures (dec_cmp_nat(a, b) == Ordering::Eq) <==> (a == b)
{
}

} // verus!