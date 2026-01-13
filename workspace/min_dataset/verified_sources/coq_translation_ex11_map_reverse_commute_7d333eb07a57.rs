use vstd::prelude::*;

verus! {

pub type IntList = Seq<int>;

pub open spec fn reverse(xs: IntList) -> IntList
    decreases xs.len()
{
    if xs.len() == 0 {
        Seq::empty()
    } else {
        reverse(xs.drop_first()).push(xs.first())
    }
}

proof fn reverse_len(xs: IntList)
    ensures reverse(xs).len() == xs.len()
    decreases xs.len()
{
    if xs.len() > 0 {
        reverse_len(xs.drop_first());
    }
}

pub proof fn lemma_reverse_index(xs: IntList, i: int)
    requires 0 <= i < xs.len()
    ensures reverse(xs).len() == xs.len(), reverse(xs)[i] == xs[xs.len() - 1 - i]
    decreases xs.len()
{
    reverse_len(xs);
    if xs.len() > 0 {
        reverse_len(xs.drop_first());
        if i == xs.len() - 1 {
        } else {
            lemma_reverse_index(xs.drop_first(), i);
        }
    }
}

pub proof fn ex_rev_reverse(xs: IntList)
    ensures reverse(reverse(xs)) =~= xs
    decreases xs.len()
{
    reverse_len(xs);
    reverse_len(reverse(xs));
    assert forall|i: int| 0 <= i < xs.len() implies reverse(reverse(xs))[i] == xs[i] by {
        lemma_reverse_index(xs, xs.len() - 1 - i);
        lemma_reverse_index(reverse(xs), i);
    }
}

} // verus!
