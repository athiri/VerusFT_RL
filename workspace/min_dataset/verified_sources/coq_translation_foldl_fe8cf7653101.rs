use vstd::prelude::*;

verus! {

pub open spec fn foldl<A, B>(xs: Seq<A>, init: B, f: spec_fn(B, A) -> B) -> B
    decreases xs.len()
{
    if xs.len() == 0 {
        init
    } else {
        foldl(xs.skip(1), f(init, xs[0]), f)
    }
}

} // verus!