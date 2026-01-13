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

pub open spec fn length<A>(xs: List<A>) -> nat {
    xs.len()
}

pub proof fn map_len<A, B>(xs: Seq<A>, f: spec_fn(A) -> B)
    ensures map(xs, f).len() == xs.len()
    decreases xs.len()
{
    if xs.len() > 0 {
        map_len(xs.drop_last(), f);
    }
}

pub proof fn ex3_length_map<A, B>(xs: List<A>, f: spec_fn(A) -> B)
    ensures length(map(xs, f)) == length(xs)
{
    map_len(xs, f);
}

} // verus!
