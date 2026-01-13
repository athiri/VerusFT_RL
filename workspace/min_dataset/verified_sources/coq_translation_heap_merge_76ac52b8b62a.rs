use vstd::prelude::*;

verus! {

pub enum Heap {
    E,  // Empty
    T { priority: nat, left: Box<Heap>, right: Box<Heap> },
}


pub open spec fn heap_merge(h1: Heap, h2: Heap) -> Heap
    decreases h1, h2
{
    match (h1, h2) {
        (Heap::E, h) => h,
        (h, Heap::E) => h,
        (Heap::T { priority: p1, left: l1, right: r1 },
         Heap::T { priority: p2, left: l2, right: r2 }) =>
            if p1 <= p2 {
                Heap::T {
                    priority: p1,
                    left: Box::new(heap_merge(*r1, h2)),
                    right: l1,
                }
            } else {
                Heap::T {
                    priority: p2,
                    left: Box::new(heap_merge(h1, *r2)),
                    right: l2,
                }
            }
    }
}

} // verus!