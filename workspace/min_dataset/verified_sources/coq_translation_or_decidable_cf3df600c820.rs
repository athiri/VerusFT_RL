use vstd::prelude::*;

verus! {

pub open spec fn decidable(p: bool) -> bool {
    p || !p  // Law of excluded middle
}


pub proof fn or_decidable(p: bool, q: bool)
    ensures decidable(p || q)
{
}

} // verus!