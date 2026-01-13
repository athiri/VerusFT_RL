use vstd::prelude::*;

verus! {

pub enum Stack<T> {
    Empty,
    Push { top: T, rest: Box<Stack<T>> },
}


pub open spec fn stack_to_seq<T>(s: Stack<T>) -> Seq<T>
    decreases s
{
    match s {
        Stack::Empty => Seq::empty(),
        Stack::Push { top, rest } => stack_to_seq(*rest).push(top),
    }
}

} // verus!