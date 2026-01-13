use vstd::prelude::*;

verus! {

pub open spec fn oneof_flatten<T>(nested: Seq<Seq<Set<T>>>) -> Set<T> {
    Set::new(|x: T|
        exists|i: int, j: int|
            0 <= i < nested.len() &&
            0 <= j < nested[i].len() &&
            nested[i][j].contains(x)
    )
}


pub proof fn oneof_flatten_contains<T>(nested: Seq<Seq<Set<T>>>, i: int, j: int, x: T)
    requires
        0 <= i < nested.len(),
        0 <= j < nested[i].len(),
        nested[i][j].contains(x),
    ensures oneof_flatten(nested).contains(x)
{
}

} // verus!