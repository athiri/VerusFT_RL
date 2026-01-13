use vstd::prelude::*;

verus! {

pub type Key = nat;

pub type PartialMap = Map<Key, int>;

pub open spec fn p_empty() -> PartialMap {
    Map::<Key, int>::empty()
}

pub open spec fn p_apply(m: PartialMap, k: Key) -> Option<int> {
    if m.dom().contains(k) { Option::Some(m[k]) } else { Option::<int>::None }
}


pub proof fn ex6_p_empty_none(k: Key)
    ensures p_apply(p_empty(), k) == Option::<int>::None
{
    // dom(empty) is empty
    assert(Map::<Key, int>::empty().dom() == Set::<Key>::empty());
    assert(!p_empty().dom().contains(k));
}

} // verus!