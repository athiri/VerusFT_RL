use vstd::prelude::*;

verus! {

pub open spec fn fst<A, B>(p: (A, B)) -> A { p.0 }


pub proof fn fst_pair<A, B>(a: A, b: B) ensures fst((a, b)) == a {}

} // verus!