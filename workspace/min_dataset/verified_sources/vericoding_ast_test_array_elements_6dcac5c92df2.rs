use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
pub open spec fn test_array_elements_postcond (a : & Vec < i32 > , j : usize , result : & Vec < i32 >) -> bool { &&& result [j as int] == 60 &&& forall | k : int | 0 <= k < a . len () && k != j ==> result [k] == a [k] &&& result . len () == a . len () }
pub open spec fn test_array_elements_precond (a : & Vec < i32 > , j : usize) -> bool { j < a . len () }
pub fn test_array_elements (a : & Vec < i32 > , j : usize) -> (result : Vec < i32 >) requires test_array_elements_precond (a , j) ensures test_array_elements_postcond (a , j , & result) { let mut result = a . clone () ; result . set (j , 60) ; result }

} // verus!