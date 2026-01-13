use vstd::prelude::*;

verus! {

pub open spec fn geb(x: nat, y: nat) -> bool {
    x >= y
}


pub proof fn geb_reflect(x: nat, y: nat)
    ensures geb(x, y) <==> x >= y
{
}

} // verus!