use vstd::prelude::*;

verus! {

pub enum Stack<T> {
    Empty,
    Push { top: T, rest: Box<Stack<T>> },
}

pub open spec fn stack_size<T>(s: Stack<T>) -> nat
    decreases s
{
    match s {
        Stack::Empty => 0,
        Stack::Push { top: _, rest } => 1 + stack_size(*rest),
    }
}

pub open spec fn stack_push<T>(x: T, s: Stack<T>) -> Stack<T> {
    Stack::Push { top: x, rest: Box::new(s) }
}


pub proof fn size_push<T>(x: T, s: Stack<T>)
    ensures stack_size(stack_push(x, s)) == stack_size(s) + 1
{
}

} // verus!