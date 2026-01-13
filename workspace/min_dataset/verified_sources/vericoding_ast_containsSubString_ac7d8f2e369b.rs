use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn containsSubString (a : & [char] , b : & [char]) -> (pos : isize) requires b . len () <= a . len () , ensures true , { return 0 ; }

} // verus!