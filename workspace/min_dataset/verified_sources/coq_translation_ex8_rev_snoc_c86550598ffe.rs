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

pub proof fn ex9_rev_app_distr(xs: NatList, ys: NatList)
    ensures xs.add(ys).reverse() =~= ys.reverse().add(xs.reverse())
{
    let m: int = xs.len() as int;
    let n: int = ys.len() as int;
    let total: int = m + n;

    assert(xs.add(ys).len() == (total as nat));
    assert(ys.reverse().add(xs.reverse()).len() == (total as nat));

    assert forall|i: int| 0 <= i < total implies xs.add(ys).reverse()[i] == ys.reverse().add(xs.reverse())[i] by {
        // LHS: reverse(xs ++ ys)[i] = (xs ++ ys)[total-1-i]
        lemma_reverse_index(xs.add(ys), i);
        assert(xs.add(ys).reverse()[i] == xs.add(ys)[total - 1 - i]);

        if i < n {
            // Then total-1-i >= m, so index is in the ys part.
            assert(total - 1 - i >= m);
            assert(xs.add(ys)[total - 1 - i] == ys[(total - 1 - i) - m]);
            assert((total - 1 - i) - m == n - 1 - i);

            // RHS index i is in ys.reverse.
            lemma_reverse_index(ys, i);
            assert(ys.reverse()[i] == ys[n - 1 - i]);
            assert(ys.reverse().add(xs.reverse())[i] == ys.reverse()[i]);
        } else {
            // Then i-n is a valid index into xs.reverse.
            assert(i - n >= 0);
            assert(i - n < m);

            // total-1-i < m, so index is in the xs part.
            assert(total - 1 - i < m);
            assert(xs.add(ys)[total - 1 - i] == xs[total - 1 - i]);
            assert(total - 1 - i == m - 1 - (i - n));

            // RHS index i is in the xs.reverse part.
            lemma_reverse_index(xs, i - n);
            assert(ys.reverse().add(xs.reverse())[i] == xs.reverse()[i - n]);
            assert(xs.reverse()[i - n] == xs[m - 1 - (i - n)]);
        }
    };

    assert(xs.add(ys).reverse() =~= ys.reverse().add(xs.reverse()));
}


pub proof fn ex8_rev_snoc(xs: NatList, v: nat)
    ensures xs.push(v).reverse() =~= seq![v].add(xs.reverse())
{
    // One convenient route: prove the stronger concat lemma (next) and instantiate.
    ex9_rev_app_distr(xs, seq![v]);

    // reverse(xs ++ [v]) = reverse([v]) ++ reverse(xs)
    assert(xs.add(seq![v]).reverse() =~= seq![v].reverse().add(xs.reverse()));
    reveal_with_fuel(Seq::reverse, 1);
    assert(seq![v].reverse() =~= seq![v]);
}

} // verus!