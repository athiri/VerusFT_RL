use vstd::prelude::*;

verus! {

pub enum Heap {
    E,  // Empty
    T { priority: nat, left: Box<Heap>, right: Box<Heap> },
}

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

} // verus!