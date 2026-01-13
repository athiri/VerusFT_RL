use vstd::prelude::*;

verus! {

pub enum Stack<T> {
    Empty,
    Push { top: T, rest: Box<Stack<T>> },
}

pub open spec fn stack_is_empty<T>(s: Stack<T>) -> bool {
    matches!(s, Stack::Empty)
}

pub open spec fn stack_empty<T>() -> Stack<T> {
    Stack::Empty
}


pub proof fn empty_is_empty<T>()
    ensures stack_is_empty(stack_empty::<T>())
{
}

} // verus!