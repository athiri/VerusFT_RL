use vstd::prelude::*;

verus! {

pub type List<A> = Seq<A>;

pub proof fn lemma_decompose_head_tail<A>(xs: List<A>)
    requires xs.len() > 0,
    ensures xs =~= seq![xs[0]].add(xs.skip(1))
{
    let tail = xs.skip(1);
    assert(seq![xs[0]].len() == 1);
    assert(tail.len() + 1 == xs.len());

    assert forall|i: int| 0 <= i < xs.len() implies seq![xs[0]].add(tail)[i] == xs[i] by {
        if i == 0 {
            assert(seq![xs[0]].add(tail)[0] == xs[0]);
        } else {
            assert(0 < i);
            assert(i - 1 >= 0);
            assert(i - 1 < tail.len());
            assert(seq![xs[0]].add(tail)[i] == tail[i - 1]);
            assert(tail[i - 1] == xs[i]);
        }
    };

    assert(seq![xs[0]].add(tail) =~= xs);
}

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


pub proof fn ex5_filter_true<A>(xs: List<A>)
    ensures filter(xs, |a: A| true) =~= xs
    decreases xs.len()
{
    if xs.len() == 0 {
        assert(filter(xs, |a: A| true) =~= Seq::empty());
        assert(xs =~= Seq::empty());
    } else {
        let tail = xs.skip(1);
        ex5_filter_true(tail);
        lemma_decompose_head_tail(xs);
        assert(filter(xs, |a: A| true) == seq![xs[0]].add(filter(tail, |a: A| true)));
        assert(filter(xs, |a: A| true) =~= seq![xs[0]].add(tail));
    }
}

} // verus!