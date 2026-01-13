use vstd::prelude::*;

verus! {

pub open spec fn arbitrary_list<A>(elements: Set<A>, max_len: nat) -> Set<Seq<A>> {
    Set::new(|s: Seq<A>| s.len() <= max_len && forall|i: int| 0 <= i < s.len() ==> elements.contains(s[i]))
}


pub proof fn empty_list_arbitrary<A>(elements: Set<A>, max_len: nat)
    ensures arbitrary_list(elements, max_len).contains(Seq::empty())
{
}

} // verus!