use vstd::prelude::*;

verus! {

pub enum Stack<T> {
    Empty,
    Push { top: T, rest: Box<Stack<T>> },
}

pub open spec fn stack_peek<T>(s: Stack<T>) -> Option<T> {
    match s {
        Stack::Empty => None,
        Stack::Push { top, rest: _ } => Some(top),
    }
}

pub open spec fn stack_push<T>(x: T, s: Stack<T>) -> Stack<T> {
    Stack::Push { top: x, rest: Box::new(s) }
}


pub proof fn peek_push<T>(x: T, s: Stack<T>)
    ensures stack_peek(stack_push(x, s)) == Some(x)
{
}

} // verus!