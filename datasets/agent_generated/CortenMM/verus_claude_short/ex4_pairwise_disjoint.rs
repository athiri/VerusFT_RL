// Source: verification/vstd_extra/src/set_extra.rs (lines 140-145)
// Concept: A spec function defining pairwise disjointness of a set of sets
// Idea: Demonstrates how to express a property about collections of collections,
//       used to ensure non-overlapping partitions in memory management
//
// The spec function `pairwise_disjoint` checks if all distinct sets in a collection
// are disjoint. The executable function `count_disjoint_elements` sums the sizes
// of two arrays representing disjoint sets.

use vstd::prelude::*;

verus! {

pub open spec fn pairwise_disjoint<A>(sets: Set<Set<A>>) -> bool {
    forall|s1: Set<A>, s2: Set<A>|
        #![trigger sets.contains(s1), sets.contains(s2)]
        sets.contains(s1) && sets.contains(s2) && s1 != s2 ==> s1.disjoint(s2)
}

pub fn count_disjoint_elements(arr1: &[u64], arr2: &[u64]) -> (res: usize)
    requires
        arr1@.len() < 1000000,
        arr2@.len() < 1000000,
        pairwise_disjoint(set![arr1@.to_set(), arr2@.to_set()]),
    ensures
        res == arr1@.len() + arr2@.len(),
{
    arr1.len() + arr2.len()
}

fn main() {}

} // verus!
