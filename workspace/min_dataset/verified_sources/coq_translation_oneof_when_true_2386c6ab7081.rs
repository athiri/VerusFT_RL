use vstd::prelude::*;

verus! {

pub open spec fn oneof_when<T>(cond: bool, then_gen: Set<T>, else_gen: Set<T>) -> Set<T> {
    if cond { then_gen } else { else_gen }
}


pub proof fn oneof_when_true<T>(then_gen: Set<T>, else_gen: Set<T>)
    ensures oneof_when(true, then_gen, else_gen) =~= then_gen
{
}

} // verus!