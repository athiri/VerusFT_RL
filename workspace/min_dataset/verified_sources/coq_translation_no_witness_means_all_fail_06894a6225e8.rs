use vstd::prelude::*;

verus! {

pub open spec fn has_witness<A>(gen: Set<A>, pred: spec_fn(A) -> bool) -> bool {
    exists|a: A| gen.contains(a) && pred(a)
}


pub proof fn no_witness_means_all_fail<A>(gen: Set<A>, pred: spec_fn(A) -> bool)
    requires !has_witness(gen, pred)
    ensures forall|a: A| gen.contains(a) ==> !pred(a)
{
}

} // verus!