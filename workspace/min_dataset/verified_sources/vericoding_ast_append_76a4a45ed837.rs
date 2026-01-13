use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn append (a : & [int] , b : int) -> (c : Vec < int >) requires a . len () < MAX ensures c @ == a @ + seq ! [b] { let mut c = Vec :: new () ; for i in 0 .. a . len () invariant c @ == a @ . subrange (0 , i as int) { c . push (a [i]) ; } c . push (b) ; c }

} // verus!