use vstd::prelude::*;

verus! {

pub open spec fn elements_outputs<T>(elems: Seq<T>) -> Set<T> {
    Set::new(|x: T| exists|i: int| 0 <= i < elems.len() && elems[i] == x)
}


pub proof fn elements_exact<T>(elems: Seq<T>, x: T)
    requires elements_outputs(elems).contains(x)
    ensures exists|i: int| 0 <= i < elems.len() && elems[i] == x
{
}

} // verus!