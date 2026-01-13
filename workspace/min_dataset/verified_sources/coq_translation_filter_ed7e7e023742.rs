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

} // verus!