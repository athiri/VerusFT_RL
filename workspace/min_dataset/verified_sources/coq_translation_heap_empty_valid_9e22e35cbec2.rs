use vstd::prelude::*;

verus! {

pub open spec fn all_ge(h: Heap, bound: nat) -> bool
    decreases h
{
    match h {
        Heap::E => true,
        Heap::T { priority, left, right } =>
            priority >= bound &&
            all_ge(*left, bound) &&
            all_ge(*right, bound),
    }
}


pub enum Heap {
    E,  // Empty
    T { priority: nat, left: Box<Heap>, right: Box<Heap> },
}

pub open spec fn is_heap(h: Heap) -> bool
    decreases h
{
    match h {
        Heap::E => true,
        Heap::T { priority, left, right } =>
            all_ge(*left, priority) &&
            all_ge(*right, priority) &&
            is_heap(*left) &&
            is_heap(*right),
    }
}

pub open spec fn heap_empty() -> Heap {
    Heap::E
}


pub proof fn heap_empty_valid()
    ensures is_heap(heap_empty())
{
}

} // verus!