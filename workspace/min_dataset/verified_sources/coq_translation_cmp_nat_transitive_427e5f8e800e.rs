use vstd::prelude::*;

verus! {

pub open spec fn cmp_nat(a: nat, b: nat) -> Ordering {
    if a < b { Ordering::Lt }
    else if a == b { Ordering::Eq }
    else { Ordering::Gt }
}


pub enum Ordering {
    Lt,
    Eq,
    Gt,
}


pub proof fn cmp_nat_transitive(x: nat, y: nat, z: nat)
    requires cmp_nat(x, y) == Ordering::Lt, cmp_nat(y, z) == Ordering::Lt
    ensures cmp_nat(x, z) == Ordering::Lt
{
}

} // verus!