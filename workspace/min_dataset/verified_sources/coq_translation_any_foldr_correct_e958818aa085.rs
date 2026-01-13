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


pub open spec fn any_foldr<A>(xs: Seq<A>, p: spec_fn(A) -> bool) -> bool {
    foldr(xs, false, |a: A, acc: bool| p(a) || acc)
}


pub proof fn any_foldr_correct<A>(xs: Seq<A>, p: spec_fn(A) -> bool)
    ensures any_foldr(xs, p) <==> exists|i: int| 0 <= i < xs.len() as int && p(xs[i])
    decreases xs.len()
{
    if xs.len() == 0 {
        assert(!any_foldr(xs, p));
    } else {
        any_foldr_correct(xs.skip(1), p);
        assert(any_foldr(xs, p) == (p(xs[0]) || any_foldr(xs.skip(1), p)));

        if p(xs[0]) {
            assert(p(xs[0]));
        }

        assert forall|i: int| 0 <= i < xs.len() as int && p(xs[i])
            implies any_foldr(xs, p) by {
            if i == 0 {
                assert(p(xs[0]));
            } else {
                assert(0 <= i - 1 < xs.skip(1).len() as int);
                assert(xs.skip(1)[i - 1] == xs[i]);
                assert(p(xs.skip(1)[i - 1]));
            }
        };
    }
}

} // verus!