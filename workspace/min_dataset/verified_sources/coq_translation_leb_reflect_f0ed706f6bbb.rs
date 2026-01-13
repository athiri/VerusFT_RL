use vstd::prelude::*;

verus! {

pub open spec fn leb(x: nat, y: nat) -> bool {
    x <= y
}


pub proof fn leb_reflect(x: nat, y: nat)
    ensures leb(x, y) <==> x <= y
{
}

} // verus!