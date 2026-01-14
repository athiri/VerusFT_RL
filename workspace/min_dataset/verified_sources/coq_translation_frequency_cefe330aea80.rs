use vstd::prelude::*;

verus! {

pub struct WeightedGen<#[verifier::reject_recursive_types] A> {
    pub weight: nat,
    pub gen: Set<A>,
}


pub open spec fn frequency<A>(gens: Seq<WeightedGen<A>>) -> Set<A>
    decreases gens.len()
{
    if gens.len() == 0 { Set::empty() }
    else { gens[0].gen.union(frequency(gens.drop_first())) }
}

} // verus!