use vstd::prelude::*;

verus! {

pub proof fn modus_ponens(p: bool, q: bool)
    requires p, p ==> q
    ensures q
{
}

} // verus!