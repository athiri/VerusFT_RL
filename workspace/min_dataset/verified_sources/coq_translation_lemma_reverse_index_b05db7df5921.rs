use vstd::prelude::*;

verus! {

pub proof fn lemma_reverse_index<A>(s: Seq<A>, i: int)
    requires 0 <= i < s.len(),
    ensures s.reverse()[i] == s[s.len() - 1 - i]
{
    reveal_with_fuel(Seq::reverse, 1);
    assert(s.reverse()[i] == s[s.len() - 1 - i]);
}

} // verus!