use vstd::prelude::*;

verus! {

pub enum Heap {
    E,  // Empty
    T { priority: nat, left: Box<Heap>, right: Box<Heap> },
}


pub open spec fn heap_size(h: Heap) -> nat
    decreases h
{
    match h {
        Heap::E => 0,
        Heap::T { priority: _, left, right } =>
            1 + heap_size(*left) + heap_size(*right),
    }
}

} // verus!