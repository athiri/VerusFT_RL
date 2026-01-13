use vstd::prelude::*;

verus! {

pub open spec fn map<A, B>(xs: Seq<A>, f: spec_fn(A) -> B) -> Seq<B>
    decreases xs.len()
{
    if xs.len() == 0 {
        Seq::empty()
    } else {
        map(xs.drop_last(), f).push(f(xs.last()))
    }
}

pub type List<A> = Seq<A>;

pub proof fn ex1_map_id<A>(xs: List<A>)
    ensures map(xs, |a: A| a) =~= xs
    decreases xs.len()
{
    if xs.len() > 0 {
        ex1_map_id(xs.drop_last());
    }
}

} // verus!
