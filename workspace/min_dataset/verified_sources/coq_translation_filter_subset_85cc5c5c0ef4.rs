use vstd::prelude::*;

verus! {

pub open spec fn gen_filter<A>(gen: Set<A>, pred: spec_fn(A) -> bool) -> Set<A> {
    Set::new(|a: A| gen.contains(a) && pred(a))
}


pub proof fn filter_subset<A>(gen: Set<A>, pred: spec_fn(A) -> bool, a: A)
    requires gen_filter(gen, pred).contains(a)
    ensures gen.contains(a)
{
}

} // verus!