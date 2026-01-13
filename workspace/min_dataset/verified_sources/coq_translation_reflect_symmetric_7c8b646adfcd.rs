use vstd::prelude::*;

verus! {

pub open spec fn reflect(b: bool, p: bool) -> bool {
    b <==> p
}


pub proof fn reflect_symmetric(b: bool, p: bool)
    ensures reflect(b, p) == reflect(p, b)
{
}

} // verus!