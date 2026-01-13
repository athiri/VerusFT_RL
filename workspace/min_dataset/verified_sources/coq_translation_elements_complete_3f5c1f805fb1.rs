use vstd::prelude::*;

verus! {

pub open spec fn elements_outputs<T>(elems: Seq<T>) -> Set<T> {
    Set::new(|x: T| exists|i: int| 0 <= i < elems.len() && elems[i] == x)
}


pub proof fn elements_complete<T>(elems: Seq<T>, i: int)
    requires 0 <= i < elems.len()
    ensures elements_outputs(elems).contains(elems[i])
{
}

} // verus!