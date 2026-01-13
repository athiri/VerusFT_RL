use vstd::prelude::*;

verus! {

pub open spec fn leb(a: nat, b: nat) -> bool {
    a <= b
}


pub proof fn leb_total(a: nat, b: nat)
    ensures leb(a, b) || leb(b, a)
{
}

} // verus!