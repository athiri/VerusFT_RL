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


pub proof fn fold_fusion<A, B, C>(
    xs: Seq<A>,
    init: B,
    f: spec_fn(A, B) -> B,
    g: spec_fn(A, C) -> C,
    h: spec_fn(B) -> C
)
    requires forall|a: A, b: B| #[trigger] h(f(a, b)) == g(a, h(b))
    ensures h(foldr(xs, init, f)) == foldr(xs, h(init), g)
    decreases xs.len()
{
    if xs.len() == 0 {
        assert(foldr(xs, init, f) == init);
        assert(foldr(xs, h(init), g) == h(init));
    } else {
        fold_fusion(xs.skip(1), init, f, g, h);
        assert(h(foldr(xs.skip(1), init, f)) == foldr(xs.skip(1), h(init), g));

        assert(foldr(xs, init, f) == f(xs[0], foldr(xs.skip(1), init, f)));
        assert(h(foldr(xs, init, f)) == h(f(xs[0], foldr(xs.skip(1), init, f))));
        assert(h(f(xs[0], foldr(xs.skip(1), init, f))) == g(xs[0], h(foldr(xs.skip(1), init, f))));
        assert(g(xs[0], h(foldr(xs.skip(1), init, f))) == g(xs[0], foldr(xs.skip(1), h(init), g)));
        assert(foldr(xs, h(init), g) == g(xs[0], foldr(xs.skip(1), h(init), g)));
    }
}

} // verus!