use vstd::prelude::*;

verus! {

pub open spec fn leb(a: nat, b: nat) -> bool {
    a <= b
}


pub proof fn leb_refl(a: nat)
    ensures leb(a, a)
{
}

} // verus!