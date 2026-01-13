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

} // verus!