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


pub proof fn ex6_filter_false<A>(xs: List<A>)
    ensures filter(xs, |a: A| false) =~= Seq::empty()
    decreases xs.len()
{
    if xs.len() == 0 {
        assert(filter(xs, |a: A| false) =~= Seq::empty());
    } else {
        let tail = xs.skip(1);
        ex6_filter_false(tail);
        assert(filter(xs, |a: A| false) == filter(tail, |a: A| false));
    }
}

} // verus!