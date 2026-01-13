use vstd::prelude::*;

verus! {

pub proof fn material_impl(p: bool, q: bool)
    ensures (p ==> q) <==> (!p || q)
{
}

} // verus!