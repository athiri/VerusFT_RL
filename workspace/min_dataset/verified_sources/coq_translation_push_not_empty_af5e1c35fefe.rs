use vstd::prelude::*;

verus! {

pub enum Stack<T> {
    Empty,
    Push { top: T, rest: Box<Stack<T>> },
}

pub open spec fn stack_push<T>(x: T, s: Stack<T>) -> Stack<T> {
    Stack::Push { top: x, rest: Box::new(s) }
}

pub open spec fn stack_is_empty<T>(s: Stack<T>) -> bool {
    matches!(s, Stack::Empty)
}


pub proof fn push_not_empty<T>(x: T, s: Stack<T>)
    ensures !stack_is_empty(stack_push(x, s))
{
}

} // verus!