use vstd::prelude::*;

verus! {

pub open spec fn oneof_outputs<T>(gens: Seq<Set<T>>) -> Set<T> {
    Set::new(|x: T|
        exists|i: int| 0 <= i < gens.len() && gens[i].contains(x)
    )
}


pub proof fn oneof_contains_each<T>(gens: Seq<Set<T>>, i: int, x: T)
    requires
        0 <= i < gens.len(),
        gens[i].contains(x),
    ensures oneof_outputs(gens).contains(x)
{
}

} // verus!