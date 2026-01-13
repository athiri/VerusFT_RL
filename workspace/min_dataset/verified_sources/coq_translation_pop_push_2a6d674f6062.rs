use vstd::prelude::*;

verus! {

pub enum Stack<T> {
    Empty,
    Push { top: T, rest: Box<Stack<T>> },
}

pub open spec fn stack_push<T>(x: T, s: Stack<T>) -> Stack<T> {
    Stack::Push { top: x, rest: Box::new(s) }
}

pub open spec fn stack_pop<T>(s: Stack<T>) -> Stack<T> {
    match s {
        Stack::Empty => Stack::Empty,
        Stack::Push { top: _, rest } => *rest,
    }
}


pub proof fn pop_push<T>(x: T, s: Stack<T>)
    ensures stack_pop(stack_push(x, s)) == s
{
}

} // verus!