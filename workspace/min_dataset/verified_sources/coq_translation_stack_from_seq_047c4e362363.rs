use vstd::prelude::*;

verus! {

pub enum Stack<T> {
    Empty,
    Push { top: T, rest: Box<Stack<T>> },
}

pub open spec fn stack_push<T>(x: T, s: Stack<T>) -> Stack<T> {
    Stack::Push { top: x, rest: Box::new(s) }
}


pub open spec fn stack_from_seq<T>(s: Seq<T>) -> Stack<T>
    decreases s.len()
{
    if s.len() == 0 {
        Stack::Empty
    } else {
        stack_push(s[s.len() - 1], stack_from_seq(s.take(s.len() - 1)))
    }
}

} // verus!