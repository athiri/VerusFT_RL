use vstd::prelude::*;

verus! {

pub proof fn modus_tollens(p: bool, q: bool)
    requires !q, p ==> q
    ensures !p
{
}

} // verus!