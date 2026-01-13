use vstd::prelude::*;

verus! {

pub proof fn lemma_reverse_index<A>(s: Seq<A>, i: int)
    requires 0 <= i < s.len(),
    ensures s.reverse()[i] == s[s.len() - 1 - i]
{
    reveal_with_fuel(Seq::reverse, 1);
    assert(s.reverse()[i] == s[s.len() - 1 - i]);
}

pub type NatList = Seq<nat>;


pub proof fn ex7_rev_involutive(xs: NatList)
    ensures xs.reverse().reverse() =~= xs
{
    // Prove by extensional equality (same length, same indexing).
    assert(xs.reverse().len() == xs.len());
    assert(xs.reverse().reverse().len() == xs.len());

    assert forall|i: int| 0 <= i < xs.len() implies xs.reverse().reverse()[i] == xs[i] by {
        lemma_reverse_index(xs.reverse(), i);
        lemma_reverse_index(xs, xs.len() - 1 - i);

        // Expand the two reverses.
        assert(xs.reverse().reverse()[i] == xs.reverse()[xs.len() - 1 - i]);
        assert(xs.reverse()[xs.len() - 1 - i] == xs[xs.len() - 1 - (xs.len() - 1 - i)]);
        assert(xs.len() - 1 - (xs.len() - 1 - i) == i);
    };

    assert(xs.reverse().reverse() =~= xs);
}

} // verus!