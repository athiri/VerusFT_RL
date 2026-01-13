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


pub proof fn foldr_unfold<A, B>(xs: Seq<A>, init: B, f: spec_fn(A, B) -> B)
    requires xs.len() > 0
    ensures foldr(xs, init, f) == f(xs[0], foldr(xs.skip(1), init, f))
{
    // Trivially true by definition
}

} // verus!