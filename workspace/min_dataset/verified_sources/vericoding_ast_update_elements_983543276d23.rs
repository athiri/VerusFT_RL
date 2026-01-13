use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn update_elements_postcond (a : & Vec < i32 > , result : & Vec < i32 >) -> bool { &&& result . len () == a . len () &&& result [4] == a [4] + 3 &&& result [7] == 516 &&& forall | i : int | 0 <= i < a . len () && i != 4 && i != 7 ==> result [i] == a [i] }
spec fn update_elements_precond (a : & Vec < i32 >) -> bool { a . len () >= 8 }
fn update_elements (a : Vec < i32 >) -> (result : Vec < i32 >) requires update_elements_precond (& a) , a [4] < MAX - 3 , a [4] > MIN ensures update_elements_postcond (& a , & result) { let mut result = a ; result . set (4 , result [4] + 3) ; result . set (7 , 516) ; result }

} // verus!