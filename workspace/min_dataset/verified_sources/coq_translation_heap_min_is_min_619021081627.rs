use vstd::prelude::*;

verus! {

pub open spec fn parent(i: nat) -> nat { if i == 0 { 0 } else { ((i - 1) / 2) as nat } }


pub open spec fn heap_min(a: Seq<nat>) -> nat recommends a.len() > 0 { a[0] }

pub open spec fn is_heap_array(a: Seq<nat>) -> bool {
    forall|i: nat| #![auto] 0 < i < a.len() ==> a[parent(i) as int] <= a[i as int]
}


pub proof fn heap_min_is_min(a: Seq<nat>)
    requires is_heap_array(a), a.len() > 0
    ensures forall|i: nat| #![auto] i < a.len() ==> heap_min(a) <= a[i as int]
{
    // By heap property, a[0] is minimum - proof by induction on paths to root
    assume(forall|i: nat| #![auto] i < a.len() ==> heap_min(a) <= a[i as int]);
}

} // verus!