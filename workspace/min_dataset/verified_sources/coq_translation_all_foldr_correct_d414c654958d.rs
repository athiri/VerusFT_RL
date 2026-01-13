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


pub open spec fn all_foldr<A>(xs: Seq<A>, p: spec_fn(A) -> bool) -> bool {
    foldr(xs, true, |a: A, acc: bool| p(a) && acc)
}


pub proof fn all_foldr_correct<A>(xs: Seq<A>, p: spec_fn(A) -> bool)
    ensures all_foldr(xs, p) <==> forall|i: int| 0 <= i < xs.len() as int ==> p(xs[i])
    decreases xs.len()
{
    if xs.len() == 0 {
        assert(all_foldr(xs, p));
    } else {
        all_foldr_correct(xs.skip(1), p);
        assert(all_foldr(xs, p) == (p(xs[0]) && all_foldr(xs.skip(1), p)));

        // Forward direction: all_foldr(xs, p) ==> forall|i| ...
        if all_foldr(xs, p) {
            assert(p(xs[0]));
            assert(all_foldr(xs.skip(1), p));
            assert forall|i: int| 0 <= i < xs.len() as int implies p(xs[i]) by {
                if i == 0 {
                    assert(p(xs[0]));
                } else {
                    assert(0 <= i - 1 < xs.skip(1).len() as int);
                    assert(xs.skip(1)[i - 1] == xs[i]);
                }
            };
        }

        // Backward direction: (forall|i| ...) ==> all_foldr(xs, p)
        if forall|i: int| 0 <= i < xs.len() as int ==> p(xs[i]) {
            assert(p(xs[0]));
            assert forall|j: int| 0 <= j < xs.skip(1).len() as int implies p(xs.skip(1)[j]) by {
                assert(xs.skip(1)[j] == xs[j + 1]);
                assert(p(xs[j + 1]));
            };
        }
    }
}

} // verus!