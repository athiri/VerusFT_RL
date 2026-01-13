use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn missing_number_postcond (nums : Seq < usize > , result : usize) -> bool { let n = nums . len () ; result <= n && ! contains (nums , result) && (forall | x : usize | # ! [trigger contains (nums , x)] x <= n && x != result ==> contains (nums , x)) }
spec fn missing_number_precond (nums : Seq < usize >) -> bool { (forall | i : int | 0 <= i < nums . len () ==> nums [i] <= nums . len ()) && (forall | i : int , j : int | 0 <= i < j < nums . len () ==> nums [i] != nums [j]) }
spec fn contains (nums : Seq < usize > , x : usize) -> bool { exists | i : int | 0 <= i < nums . len () && nums [i] == x }
# [verifier :: external_body] fn missing_number (nums : Vec < usize >) -> (result : usize) requires missing_number_precond (nums @) ensures missing_number_postcond (nums @ , result) { return 0 ; }

} // verus!