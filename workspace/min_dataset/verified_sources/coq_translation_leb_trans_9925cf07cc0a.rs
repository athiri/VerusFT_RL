use vstd::prelude::*;

verus! {

pub open spec fn leb(a: nat, b: nat) -> bool {
    a <= b
}


pub proof fn leb_trans(a: nat, b: nat, c: nat)
    requires leb(a, b), leb(b, c)
    ensures leb(a, c)
{
}

} // verus!