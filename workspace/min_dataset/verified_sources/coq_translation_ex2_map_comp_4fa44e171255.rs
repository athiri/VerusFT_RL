use vstd::prelude::*;
use vstd::seq_lib::*;

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

proof fn map_len<A, B>(xs: Seq<A>, f: spec_fn(A) -> B)
    ensures map(xs, f).len() == xs.len()
    decreases xs.len()
{
    if xs.len() > 0 {
        map_len(xs.drop_last(), f);
    }
}

proof fn map_index<A, B>(xs: Seq<A>, f: spec_fn(A) -> B, i: int)
    requires 0 <= i < xs.len()
    ensures map(xs, f)[i] == f(xs[i])
    decreases xs.len()
{
    map_len(xs, f);
    if i < xs.len() - 1 {
        map_index(xs.drop_last(), f, i);
    }
}

pub proof fn ex2_map_comp<A, B, C>(xs: List<A>, f: spec_fn(A) -> B, g: spec_fn(B) -> C)
    ensures map(map(xs, f), g) =~= map(xs, |a: A| g(f(a)))
    decreases xs.len()
{
    map_len(xs, f);
    map_len(map(xs, f), g);
    map_len(xs, |a: A| g(f(a)));
    
    assert forall|i: int| 0 <= i < xs.len() implies map(map(xs, f), g)[i] == map(xs, |a: A| g(f(a)))[i] by {
        map_index(xs, f, i);
        map_index(map(xs, f), g, i);
        map_index(xs, |a: A| g(f(a)), i);
    }
}

} // verus!
