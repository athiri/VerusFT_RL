use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn fillK (a : & [int] , n : usize , k : int , c : usize) -> (b : bool) requires c <= n , n == a . len () , ensures true , { return false ; }

} // verus!