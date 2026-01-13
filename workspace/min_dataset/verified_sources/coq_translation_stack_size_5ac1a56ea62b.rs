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

} // verus!