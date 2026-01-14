use vstd::prelude::*;

verus! {

pub struct WeightedGen<#[verifier::reject_recursive_types] A> {
    pub weight: nat,
    pub gen: Set<A>,
}


pub open spec fn total_weight<A>(gens: Seq<WeightedGen<A>>) -> nat
    decreases gens.len()
{
    if gens.len() == 0 { 0 }
    else { gens[0].weight + total_weight(gens.drop_first()) }
}

} // verus!