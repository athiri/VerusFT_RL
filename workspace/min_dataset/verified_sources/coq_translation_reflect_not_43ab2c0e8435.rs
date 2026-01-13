use vstd::prelude::*;

verus! {

pub open spec fn reflect(b: bool, p: bool) -> bool {
    b <==> p
}


pub proof fn reflect_not(b: bool, p: bool)
    requires reflect(b, p)
    ensures reflect(!b, !p)
{
}

} // verus!