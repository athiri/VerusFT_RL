use vstd::prelude::*;

verus! {

pub open spec fn reflect(b: bool, p: bool) -> bool {
    b <==> p
}


pub proof fn reflect_or(b1: bool, b2: bool, p1: bool, p2: bool)
    requires reflect(b1, p1), reflect(b2, p2)
    ensures reflect(b1 || b2, p1 || p2)
{
}

} // verus!