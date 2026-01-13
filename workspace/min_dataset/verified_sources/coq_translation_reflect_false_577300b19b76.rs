use vstd::prelude::*;

verus! {

pub open spec fn reflect(b: bool, p: bool) -> bool {
    b <==> p
}


pub proof fn reflect_false(p: bool)
    requires !p
    ensures reflect(false, p)
{
}

} // verus!