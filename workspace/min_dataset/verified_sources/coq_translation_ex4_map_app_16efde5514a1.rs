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

pub proof fn ex4_map_app<A, B>(xs: List<A>, ys: List<A>, f: spec_fn(A) -> B)
    ensures map(xs.add(ys), f) =~= map(xs, f).add(map(ys, f))
{
    map_len(xs, f);
    map_len(ys, f);
    map_len(xs.add(ys), f);
    
    assert forall|i: int| 0 <= i < xs.len() + ys.len() implies map(xs.add(ys), f)[i] == map(xs, f).add(map(ys, f))[i] by {
        map_index(xs.add(ys), f, i);
        if i < xs.len() {
            map_index(xs, f, i);
        } else {
            map_index(ys, f, i - xs.len());
        }
    }
}

} // verus!
