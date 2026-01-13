use vstd::prelude::*;

verus! {

pub open spec fn decidable(p: bool) -> bool {
    p || !p  // Law of excluded middle
}


pub proof fn not_decidable(p: bool)
    ensures decidable(!p)
{
}

} // verus!