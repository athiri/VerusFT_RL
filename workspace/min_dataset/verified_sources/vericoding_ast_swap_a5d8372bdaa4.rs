use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn swap_postcond (arr : Seq < i32 > , i : i32 , j : i32 , result : Seq < i32 >) -> bool { result [i as int] == arr [j as int] && result [j as int] == arr [i as int] && result . len () == arr . len () && forall | k : int | 0 <= k < arr . len () && k != i && k != j ==> result [k] == arr [k] }
spec fn swap_precond (arr : Seq < i32 > , i : i32 , j : i32) -> bool { i >= 0 && j >= 0 && (i as nat) < arr . len () && (j as nat) < arr . len () }
fn swap (arr : Vec < i32 > , i : i32 , j : i32) -> (result : Vec < i32 >) requires swap_precond (arr @ , i , j) , ensures swap_postcond (arr @ , i , j , result @) , { let mut result = arr ; let temp = result [i as usize] ; result . set (i as usize , result [j as usize]) ; result . set (j as usize , temp) ; result }

} // verus!