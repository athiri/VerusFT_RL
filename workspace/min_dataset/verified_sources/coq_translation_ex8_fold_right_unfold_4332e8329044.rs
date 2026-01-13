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


pub proof fn ex8_fold_right_unfold<A, B>(xs: List<A>, init: B, f: spec_fn(A, B) -> B)
    requires xs.len() > 0,
    ensures fold_right(xs, init, f) == f(xs[0], fold_right(xs.skip(1), init, f))
{
    reveal_with_fuel(fold_right, 1);
    assert(fold_right(xs, init, f) == f(xs[0], fold_right(xs.skip(1), init, f)));
}

} // verus!