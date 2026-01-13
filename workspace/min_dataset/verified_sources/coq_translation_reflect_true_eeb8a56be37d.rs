use vstd::prelude::*;

verus! {

pub open spec fn reflect(b: bool, p: bool) -> bool {
    b <==> p
}


pub proof fn reflect_true(p: bool)
    requires p
    ensures reflect(true, p)
{
}

} // verus!