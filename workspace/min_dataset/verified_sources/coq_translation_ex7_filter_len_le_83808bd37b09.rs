use vstd::prelude::*;

verus! {

pub type List<A> = Seq<A>;

pub open spec fn filter<A>(xs: List<A>, p: spec_fn(A) -> bool) -> List<A>
    decreases xs.len()
{
    if xs.len() == 0 {
        Seq::empty()
    } else {
        let x = xs[0];
        let rest = filter(xs.skip(1), p);
        if p(x) {
            seq![x].add(rest)
        } else {
            rest
        }
    }
}


pub proof fn ex7_filter_len_le<A>(xs: List<A>, p: spec_fn(A) -> bool)
    ensures filter(xs, p).len() <= xs.len()
    decreases xs.len()
{
    if xs.len() == 0 {
        assert(filter(xs, p).len() == 0);
    } else {
        let tail = xs.skip(1);
        ex7_filter_len_le(tail, p);
        if p(xs[0]) {
            assert(filter(xs, p) == seq![xs[0]].add(filter(tail, p)));
            assert(filter(xs, p).len() == filter(tail, p).len() + 1);
            assert(filter(tail, p).len() + 1 <= tail.len() + 1);
            assert(tail.len() + 1 == xs.len());
        } else {
            assert(filter(xs, p) == filter(tail, p));
            assert(filter(xs, p).len() <= tail.len());
            assert(tail.len() < xs.len());
        }
    }
}

} // verus!