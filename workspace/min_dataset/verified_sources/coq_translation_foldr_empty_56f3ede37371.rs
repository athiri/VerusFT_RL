use vstd::prelude::*;

verus! {

pub open spec fn foldr<A, B>(xs: Seq<A>, init: B, f: spec_fn(A, B) -> B) -> B
    decreases xs.len()
{
    if xs.len() == 0 {
        init
    } else {
        f(xs[0], foldr(xs.skip(1), init, f))
    }
}


pub proof fn foldr_empty<A, B>(init: B, f: spec_fn(A, B) -> B)
    ensures foldr(Seq::<A>::empty(), init, f) == init
{
    assert(Seq::<A>::empty().len() == 0);
}

} // verus!