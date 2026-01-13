use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
pub open spec fn append_postcond (a : Seq < int > , b : int , result : Seq < int >) -> bool { (forall | i : int | 0 <= i < a . len () ==> result [i] == a [i]) && result [a . len () as int] == b && result . len () == a . len () + 1 }
pub open spec fn append_precond (a : Seq < int > , b : int) -> bool { true }
fn copy (a : & Vec < int > , i : usize , acc : & mut Vec < int >) requires i <= a . len () , old (acc) . len () == i , forall | j : int | 0 <= j < i ==> old (acc) [j] == a [j] , ensures acc . len () == a . len () , forall | j : int | 0 <= j < a . len () ==> acc [j] == a [j] , decreases a . len () - i , { if i < a . len () { acc . push (a [i]) ; copy (a , i + 1 , acc) ; } }
pub fn append (a : & Vec < int > , b : int) -> (result : Vec < int >) requires append_precond (a @ , b) , ensures append_postcond (a @ , b , result @) , { let mut result = Vec :: new () ; copy (a , 0 , & mut result) ; result . push (b) ; result }

} // verus!