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


pub proof fn cmp_nat_antisymmetric(x: nat, y: nat)
    ensures cmp_nat(x, y) == Ordering::Lt <==> cmp_nat(y, x) == Ordering::Gt
{
}

} // verus!