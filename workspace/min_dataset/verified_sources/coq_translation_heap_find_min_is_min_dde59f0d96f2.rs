use vstd::prelude::*;

verus! {

pub struct Heap {
    pub data: Seq<nat>,
}

pub open spec fn is_heap(h: Heap) -> bool {
    forall|i: nat| #[trigger] h.data[i as int] == h.data[i as int] && (0 < i < h.data.len() ==> h.data[((i - 1) / 2) as int] <= h.data[i as int])
}

pub open spec fn heap_is_empty(h: Heap) -> bool {
    h.data.len() == 0
}

pub open spec fn heap_peek(h: Heap) -> nat
    recommends !heap_is_empty(h)
{
    h.data.first()
}

proof fn heap_root_le_node(h: Heap, i: nat)
    requires is_heap(h), i < h.data.len()
    ensures h.data[0] <= h.data[i as int]
    decreases i
{
    if i > 0 {
        let parent: nat = ((i - 1) / 2) as nat;
        assert(h.data[i as int] == h.data[i as int]);
        heap_root_le_node(h, parent);
    }
}

pub proof fn heap_peek_is_min(h: Heap)
    requires is_heap(h), !heap_is_empty(h)
    ensures forall|i: nat| 0 <= i < h.data.len() ==> heap_peek(h) <= h.data[i as int]
{
    assert forall|i: nat| 0 <= i < h.data.len() implies heap_peek(h) <= h.data[i as int] by {
        heap_root_le_node(h, i);
    }
}

} // verus!
