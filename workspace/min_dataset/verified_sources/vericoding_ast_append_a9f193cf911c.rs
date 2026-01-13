use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn append (a : & [int] , b : int) -> (c : Vec < int >) requires a . len () < MAX ensures c @ == a @ + seq ! [b] { let mut result = Vec :: new () ; let mut i = 0 ; while i < a . len () invariant i <= a . len () , result @ == a @ . subrange (0 , i as int) decreases a . len () - i { result . push (a [i]) ; i += 1 ; } result . push (b) ; result }

} // verus!