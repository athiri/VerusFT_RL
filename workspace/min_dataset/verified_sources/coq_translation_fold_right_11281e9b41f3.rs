use vstd::prelude::*;

verus! {

pub type List<A> = Seq<A>;


pub open spec fn fold_right<A, B>(xs: List<A>, init: B, f: spec_fn(A, B) -> B) -> B
    decreases xs.len()
{
    if xs.len() == 0 {
        init
    } else {
        f(xs[0], fold_right(xs.skip(1), init, f))
    }
}

} // verus!