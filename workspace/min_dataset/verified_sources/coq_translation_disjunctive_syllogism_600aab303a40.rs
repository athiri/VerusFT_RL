use vstd::prelude::*;

verus! {

pub proof fn disjunctive_syllogism(p: bool, q: bool)
    requires p || q, !p
    ensures q
{
}

} // verus!