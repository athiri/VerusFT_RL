use vstd::prelude::*;

verus! {

pub open spec fn stack_push<T>(x: T, s: Stack<T>) -> Stack<T> {
    Stack::Push { top: x, rest: Box::new(s) }
}


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

pub open spec fn stack_from_seq<T>(s: Seq<T>) -> Stack<T>
    decreases s.len()
{
    if s.len() == 0 {
        Stack::Empty
    } else {
        stack_push(s[s.len() - 1], stack_from_seq(s.take(s.len() - 1)))
    }
}


pub proof fn stack_seq_roundtrip<T>(s: Seq<T>)
    ensures stack_to_seq(stack_from_seq(s)) =~= s
    decreases s.len()
{
    reveal_with_fuel(stack_from_seq, 2);
    reveal_with_fuel(stack_to_seq, 2);
    if s.len() > 0 {
        stack_seq_roundtrip(s.take(s.len() - 1));
    }
}

} // verus!